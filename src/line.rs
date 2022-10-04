mod route;

struct Line {
   id: u32,
   name: String,
   segments: Vec<route::Route>,
   looping: bool,
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
            looping: false,
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
        let new_segment = route::Route::new(segment_id, &self.name, last_station, end_station);
        self.segments.push(new_segment);
        self.end_segment = Some(new_segment.id);
        &self.segments.last()
    }
    //returns next segment to go on for the train and if to switch direction
    fn get_next_segment(&self, segment_id: &u32, forward: &bool) -> (&u32, bool) {
        //self.segments.iter().filter(|segment| segment.id.eq(segment_id)).next().id
        //.find instead of .filter will be faster
        let counter = 0;
        for segment in self.segments {
            if segment_id == segment.id && forward {
                if segment == self.segments.last && self.looping == true {
                    (self.segments[counter + 1], false)
                } if segment == self.segments.last() {
                    (self.segments[counter - 1].id, true)
                } else {
                    (self.segments[counter + 1].id, false)
                }
            }
            else if segment_id == segment.id && foward == false {
                if segment == self.segments.first() && self.looping == true {
                    (self.segments[counter + 1], false)
                } if segment == self.segments.first() {
                    (self.segments[counter + 1].id, true)
                } else {
                    (self.segments[counter - 1].id, false)
                }
            }
            else { continue }
        }
    }
    fn add_segment_start(&self, segment_id: &mut u32, start_station: u32) -> &route::Route {
        *segment_id += 1;
        let old_first_station = self.segments.first().start_station;
        let new_segment = route::Route::new(segment_id, &self.name, start_station, old_first_station);
        self.segments.insert(0, new_segment);
        self.start_segment = Some(new_segment.id);
    }
    fn add_segment_middle(&self, segment_id: &mut u32, start_station: u32, end_station: u32) -> 
        &route::Route {
        *segment_id += 1;
        let mut i = 0;
        let mut index = i;
        let new_segment = route::Route::new(segment_id, &self.name, start_station, end_station)
        if (self.segments.last().end_station == start_station && 
            self.segments.first().start_station == end_station) {
                self.segments.push(new_segment);
                self.start_segment = None;
                self.end_segment = None;
                self.looping = true;
                new_segment
        }
        for segment in self.segments {
            i += 1;
            if segment.end_station == start_station { //insert new segment after existing one
                index = i + 1;
                if self.segments.get(index) == None {
                    self.add_segment_end(segment_id - 1, end_station)
                }
                self.segments.start_segment = end_station;
                segments.insert(index, new_segment);
                new_segment
            }
        }
    }
    fn merge_segments(&self, common_station: &u32) -> &route::Route {
        let counter: usize = 0;
        for segment in self.segments {
            if segment.end_station == common_station {
                segment.end_station = self.segments.get(counter + 1).end_station;
                self.segments.remove(counter + 1);
                if segment == self.segments.last() && self.looping == false {
                    self.segments.end_segment = segment.id;
                }
                segment
            }
        }
    }
    fn remove_segment(&self, segment_id: &u32) -> Result<route::Route, Err> {
        let counter: usize = 0;
        for &segment in self.segments {
            if segment_id == segment.id {
                if segment == self.segments.first() && self.looping == true {
                    self.segments.last().end_station = self.segments.get(1).start_station;
                    self.segments.get_mut(1).start_station = self.segments.last().end_station;
                    self.segments.remove[0]
                } else if segment == self.segments.first() {
                    self.start_segment = self.segments[1].id;
                    self.segments.remove[0]
                } else if segment == self.segments.last() && self.looping == true {
                    let removed_segment = self.segments.pop();
                    self.segments.first_mut().start_station = self.last().end_station;
                    self.segments.last_mut().end_station = self.first().start_station;
                    removed_segment
                } else if segment == self.segments.last() {
                    let removed_segment = self.segments.pop();
                    self.end_segment == segment.id;
                } else {
                    self.segments.get_mut(counter - 1).end_station = 
                        self.segments.get(counter + 1).start_station;
                    self.segments.get_mut(counter + 1).start_station = 
                        self.segments.get(counter - 1).end_station;
                    self.segments.remove(counter)
                }
            }
            else { counter += 1 }
        }
        return Err
    }
}
