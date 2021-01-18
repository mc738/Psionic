use crate::http::{HttpResponse, HttpRequest};
use std::collections::HashMap;
use std::path::Path;

pub struct StaticRoute {

    content_path: String,
    content_type: String
}

pub struct DynamicRoute {

}

pub struct BespokeRoute {

}

pub struct WebSocketRoute {
    
}

pub struct ProxyRoute {

}

pub enum RouteHandler {
    Static(StaticRoute),
    Dynamic(DynamicRoute),
    WebSocket(WebSocketRoute),
    Proxy(ProxyRoute),
    Bespoke(BespokeRoute)
}

pub struct Route {
    pub name: String,
    pub handler: RouteHandler
}

pub struct RouteMap {
    routes: HashMap<String, RouteHandler>
}

impl RouteHandler {
    pub fn handle(&self, request: HttpRequest) -> Result<HttpResponse, & 'static str> {
        match self {
            RouteHandler::Static(h) => unimplemented!(), /*h.handle(request),*/
            RouteHandler::Dynamic(h) => unimplemented!(),
            RouteHandler::Proxy(h) => unimplemented!(),
            RouteHandler::WebSocket(h) => unimplemented!(),
            RouteHandler::Bespoke(h) => unimplemented!(),
        }
    }
}


impl RouteMap {
    pub fn create(routes: Vec<Route>) -> Result<RouteMap, &'static str> {
        let mut map: HashMap<String, RouteHandler> = HashMap::new();
        
        for r in routes {
            map.insert(r.name, r.handler);
        };
        
        Ok(RouteMap {
            routes: map
        })
    }
    
    
    pub fn handle(&self, request: HttpRequest) -> HttpResponse {
        
        
        let response= match self.routes.get(request.header.route.as_str()) {
            Some(r) => r.handle(request),
            None => Err("")
        };
        
        response.unwrap()
    }
}

impl StaticRoute {
    pub fn create(route: String, content_path: String, content_type: String) -> StaticRoute {
        StaticRoute {
            content_path,
            content_type
        }
    }
    
   //pub fn handle(&self, request: HttpRequest) -> Result<HttpResponse, & 'static str> {
        
        // Copy static content from internal cache.
   //}
}