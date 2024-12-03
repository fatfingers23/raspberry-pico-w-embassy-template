#![no_std]
#![no_main]

use core::sync::atomic::AtomicBool;

use cyw43::{Control, JoinOptions};
use cyw43_driver::{net_task, setup_cyw43};
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::clocks::RoscRng;
use embassy_time::Timer;
use env::env_value;
use http_server::{
    HttpServer, Response, StatusCode, WebRequest, WebRequestHandler, WebRequestHandlerError,
};
use io::easy_format_str;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

mod cyw43_driver;
mod env;
mod http_server;
mod io;

pub static LIGHT_STATUS: AtomicBool = AtomicBool::new(true);

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let (net_device, mut control) = setup_cyw43(
        p.PIO0, p.PIN_23, p.PIN_24, p.PIN_25, p.PIN_29, p.DMA_CH0, spawner,
    )
    .await;

    let config = Config::dhcpv4(Default::default());
    let mut rng: RoscRng = RoscRng;

    // Generate random seed
    let seed = rng.next_u64();

    // Init network stack
    static RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let (stack, runner) = embassy_net::new(
        net_device,
        config,
        RESOURCES.init(StackResources::new()),
        seed,
    );

    unwrap!(spawner.spawn(net_task(runner)));
    let wifi_network = env_value("WIFI_SSID");
    let wifi_password = env_value("WIFI_PASSWORD");

    loop {
        match control
            .join(wifi_network, JoinOptions::new(wifi_password.as_bytes()))
            .await
        {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
            }
        }
    }

    // Wait for DHCP, not necessary when using static IP
    info!("waiting for DHCP...");
    while !stack.is_config_up() {
        Timer::after_millis(100).await;
    }
    info!("DHCP is now up!");

    info!("waiting for link up...");
    while !stack.is_link_up() {
        Timer::after_millis(500).await;
    }
    info!("Link is up!");

    info!("waiting for stack to be up...");
    stack.wait_config_up().await;
    info!("Stack is up! You are good to use the network!");

    control.gpio_set(0, true).await;

    spawner.must_spawn(http_server_task(stack, WebsiteHandler { control }));
}

#[embassy_executor::task]
async fn http_server_task(stack: Stack<'static>, website_handler: WebsiteHandler) {
    let mut server = HttpServer::new(80, stack);
    server.serve(website_handler).await;
}

struct WebsiteHandler {
    control: Control<'static>,
}

impl WebRequestHandler for WebsiteHandler {
    async fn handle_request<'a>(
        &mut self,
        request: WebRequest<'_, '_>,
        response_buffer: &'a mut [u8],
    ) -> Result<Response<'a>, WebRequestHandlerError> {
        let path = request.path.unwrap();
        match path {
            "/" => {
                let web_app = include_str!("../web_app/index.html");
                return Ok(Response::new_html(StatusCode::Ok, web_app));
            }
            "/post_test" => {
                if request.method.unwrap().as_str() == http_server::Method::Post.as_str() {
                    info!("Received body: {:?}", request.body);
                    return Ok(Response::new_html(StatusCode::Ok, "Received body"));
                }
                return Ok(Response::new_html(
                    StatusCode::MethodNotAllowed,
                    "Only POST method is allowed",
                ));
            }
            "/light_status" => {
                let light_status = LightStatus {
                    light_status: LIGHT_STATUS.load(core::sync::atomic::Ordering::Relaxed),
                };

                match serde_json_core::to_string::<_, 128>(&light_status) {
                    Ok(response) => {
                        let json_body =
                            easy_format_str(format_args!("{}", response), response_buffer);

                        Ok(Response::json_response(StatusCode::Ok, json_body.unwrap()))
                    }
                    Err(_) => Ok(Response::new_html(
                        StatusCode::InternalServerError,
                        "Error serializing json",
                    )),
                }
            }
            "/on" => {
                LIGHT_STATUS.store(true, core::sync::atomic::Ordering::Relaxed);
                self.control.gpio_set(0, true).await;
                Ok(Response::new_html(StatusCode::Ok, "Light is on"))
            }
            "/off" => {
                LIGHT_STATUS.store(false, core::sync::atomic::Ordering::Relaxed);
                self.control.gpio_set(0, false).await;
                Ok(Response::new_html(StatusCode::Ok, "Light is off"))
            }
            _ => {
                let not_found_html_response = easy_format_str(
                    format_args!(
                        "
            <!DOCTYPE html>
            <html>
                <body>                    
                    <h1>The url {path} was not found.</h1>
                </body>
            </html>
            "
                    ),
                    response_buffer,
                );

                Ok(Response::new_html(
                    StatusCode::NotFound,
                    not_found_html_response.unwrap(),
                ))
            }
        }
    }
}

///We really do not need a struct for this, but used to show sending and receiving json
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, defmt::Format)]
pub struct LightStatus {
    pub light_status: bool,
}
