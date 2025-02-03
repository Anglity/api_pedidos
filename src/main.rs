use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::sync::Arc;
use uuid::Uuid;

const FIREBASE_URL: &str = "https://appparalelo-13c98-default-rtdb.firebaseio.com/"; // 🔹 URL de Firebase
const FIREBASE_COLLECTION_PEDIDOS: &str = "pedidos";
const FIREBASE_COLLECTION_PROVEEDORES: &str = "proveedores";

// 🔹 **Estructuras de datos**
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pedido {
    id: String,
    nombre_cliente: String,
    contacto: String,
    producto: String,
    cantidad: u32,
    fecha_entrega: String,
    direccion: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Proveedor {
    id: String,
    nombre: String,
    contacto: String,
    direccion: Option<String>,
}

#[derive(Deserialize)]
struct PedidoInput {
    nombre_cliente: String,
    contacto: String,
    producto: String,
    cantidad: u32,
    fecha_entrega: String,
    direccion: Option<String>,
}

#[derive(Deserialize)]
struct ProveedorInput {
    nombre: String,
    contacto: String,
    direccion: Option<String>,
}

// 🔹 **1️⃣ Crear un pedido y guardarlo en Firebase**
async fn crear_pedido(
    client: web::Data<Arc<Client>>,
    nuevo_pedido: web::Json<PedidoInput>,
) -> impl Responder {
    let pedido_id = Uuid::new_v4().to_string();
    let pedido = Pedido {
        id: pedido_id.clone(),
        nombre_cliente: nuevo_pedido.nombre_cliente.clone(),
        contacto: nuevo_pedido.contacto.clone(),
        producto: nuevo_pedido.producto.clone(),
        cantidad: nuevo_pedido.cantidad,
        fecha_entrega: nuevo_pedido.fecha_entrega.clone(),
        direccion: nuevo_pedido.direccion.clone(),
    };

    let url = format!("{}/{}/{}.json", FIREBASE_URL, FIREBASE_COLLECTION_PEDIDOS, pedido_id);
    let response = client.put(&url).json(&pedido).send().await;

    match response {
        Ok(_) => HttpResponse::Created().json(pedido),
        Err(e) => {
            println!("❌ Error al guardar pedido en Firebase: {:?}", e);
            HttpResponse::InternalServerError().body("Error al crear pedido")
        }
    }
}

// 🔹 **2️⃣ Obtener todos los pedidos desde Firebase**
async fn obtener_pedidos(client: web::Data<Arc<Client>>) -> impl Responder {
    let url = format!("{}/{}.json", FIREBASE_URL, FIREBASE_COLLECTION_PEDIDOS);

    match client.get(&url).send().await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => {
                println!("❌ Error al parsear pedidos de Firebase: {:?}", e);
                HttpResponse::InternalServerError().body("Error al parsear pedidos")
            }
        },
        Err(e) => {
            println!("❌ Error al obtener pedidos desde Firebase: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener pedidos")
        }
    }
}

// 🔹 **3️⃣ Crear un proveedor y guardarlo en Firebase**
async fn crear_proveedor(
    client: web::Data<Arc<Client>>,
    nuevo_proveedor: web::Json<ProveedorInput>,
) -> impl Responder {
    let proveedor_id = Uuid::new_v4().to_string();
    let proveedor = Proveedor {
        id: proveedor_id.clone(),
        nombre: nuevo_proveedor.nombre.clone(),
        contacto: nuevo_proveedor.contacto.clone(),
        direccion: nuevo_proveedor.direccion.clone(),
    };

    let url = format!("{}/{}/{}.json", FIREBASE_URL, FIREBASE_COLLECTION_PROVEEDORES, proveedor_id);
    let response = client.put(&url).json(&proveedor).send().await;

    match response {
        Ok(_) => HttpResponse::Created().json(proveedor),
        Err(e) => {
            println!("❌ Error al guardar proveedor en Firebase: {:?}", e);
            HttpResponse::InternalServerError().body("Error al crear proveedor")
        }
    }
}

// 🔹 **4️⃣ Obtener todos los proveedores desde Firebase**
async fn obtener_proveedores(client: web::Data<Arc<Client>>) -> impl Responder {
    let url = format!("{}/{}.json", FIREBASE_URL, FIREBASE_COLLECTION_PROVEEDORES);

    match client.get(&url).send().await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(data) => HttpResponse::Ok().json(data),
            Err(e) => {
                println!("❌ Error al parsear proveedores de Firebase: {:?}", e);
                HttpResponse::InternalServerError().body("Error al parsear proveedores")
            }
        },
        Err(e) => {
            println!("❌ Error al obtener proveedores desde Firebase: {:?}", e);
            HttpResponse::InternalServerError().body("Error al obtener proveedores")
        }
    }
}

// 🔹 **5️⃣ Eliminar un pedido de Firebase**
async fn eliminar_pedido(client: web::Data<Arc<Client>>, id: web::Path<String>) -> impl Responder {
    let url = format!("{}/{}/{}.json", FIREBASE_URL, FIREBASE_COLLECTION_PEDIDOS, id);

    match client.delete(&url).send().await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            println!("❌ Error al eliminar pedido en Firebase: {:?}", e);
            HttpResponse::InternalServerError().body("Error al eliminar pedido")
        }
    }
}

// 🔹 **6️⃣ Eliminar un proveedor de Firebase**
async fn eliminar_proveedor(client: web::Data<Arc<Client>>, id: web::Path<String>) -> impl Responder {
    let url = format!("{}/{}/{}.json", FIREBASE_URL, FIREBASE_COLLECTION_PROVEEDORES, id);

    match client.delete(&url).send().await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            println!("❌ Error al eliminar proveedor en Firebase: {:?}", e);
            HttpResponse::InternalServerError().body("Error al eliminar proveedor")
        }
    }
}

use actix_web::middleware::Logger;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(Client::new()); // 🔹 Crear la instancia antes del servidor

    println!("🚀 Iniciando servidor Actix-Web en http://0.0.0.0:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone())) // 🔹 Compartir la instancia de Client
            .route("/api/pedidos", web::post().to(crear_pedido))
            .route("/api/pedidos", web::get().to(obtener_pedidos))
            .route("/api/pedidos/{id}", web::delete().to(eliminar_pedido))
            .route("/api/proveedores", web::post().to(crear_proveedor))
            .route("/api/proveedores", web::get().to(obtener_proveedores))
            .route("/api/proveedores/{id}", web::delete().to(eliminar_proveedor))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

