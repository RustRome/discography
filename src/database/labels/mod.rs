use ::discography::Discography;


pub struct Label {}

pub struct LabelsEndpoint<'a> {
    client: &'a Discography
}


impl<'a> LabelsEndpoint<'a> {
    pub fn new(client: &Discography) -> LabelsEndpoint {
        LabelsEndpoint { client: client }
    }


    pub fn one(&self, id: i64) -> LabelEndpoint {
        LabelEndpoint::new(self.client, id)
    }
}


pub struct LabelEndpoint<'a> {
    id: i64,
    client: &'a Discography
}

impl<'a> LabelEndpoint<'a> {
    fn new(client: &Discography, id: i64) -> LabelEndpoint {
        LabelEndpoint {
            id: id,
            client: client
        }
    }

}


