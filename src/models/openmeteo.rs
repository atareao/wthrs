


struct OpenMeteoClient{
    latitude: f32,
    longitude: f32
}

impl OpenMeteoClient{
    pub fn new(latitude: f32, longitude: f32) -> Self{
        Self{
            latitude,
            longitude
        }
    }
    pub async fn get(self){

    }
}
