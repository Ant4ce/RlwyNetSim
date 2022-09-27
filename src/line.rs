mod route;

struct Line {
   id: u32,
   name: String,
   segments: Vec<route::Route>,
   start_segment: Option<u32>,
   end_segment: Option<u32>,
}

impl Line {
    fn new(id: &mut u32, name: String) -> Line {
        *id += 1;

        Line {
            id: id.clone(),
            name,
            segments: vec![],
            start_segment: None,
            end_segment: None,
        }
    }
    //needed functions: add_segment at end, delete_station / delete_route, re_route(delete routes
    //and create new ones OR change start_/end_station (either) of two routes to connect a new
    //station), get_next_segment (for train)
    fn add_segment_end(&self, segment_id: &mut u32, end_station: u32) -> &route::Route {
        *segment_id += 1;
        let last_station = self.segments.last().end_station;
        let new_segment = route::Route::new(segment_id, &'a self.name, last_station, end_station);
        self.segments.push(new_segment);
        self.end_segment = Some(new_segment);
        &self.segments.last()
    }
    fn add_segment_middle(&self, segment_id: &mut u32, start_station: u32, end_station: u32) -> &route::Route {
        *segment_id += 1;
        for segment in self.segments {
            if segment.end_segment == start_station { //insert new segment after existing one

        }
    }
}
