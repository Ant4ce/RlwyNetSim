mod route;

enum LineError {
   RemoveSegmentError,
   NoCommonStationError,
}

struct Line<'a> {
   id: u32,
   name: String,
   segments: Vec<route::Route<'a>>,
   looping: bool,
   start_segment: Option<u32>,
   end_segment: Option<u32>,
}

impl<'a> Line<'a> {
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
    fn get_name(&self) -> &String{ &self.name }
    //needed functions: add_segment at end, delete_station / delete_route, re_route(delete routes
    //and create new ones OR change start_/end_station (either) of two routes to connect a new
    //station), get_next_segment (for train)
    fn add_segment_end<'b: 'a>(&'b mut self, segment_id: &mut u32, end_station: u32) -> &route::Route {
        *segment_id += 1;
        let last_station = self.segments.last().unwrap().end_station;
        let new_segment = route::Route::new(segment_id, &self.name, last_station, end_station);
        self.segments.push(new_segment);
        self.end_segment = Some(self.segments.last().unwrap().id);
        &self.segments.last().unwrap()
    }
    //returns next segment to go on for the train and if to switch direction
    fn get_next_segment(&'a mut self, segment_id: &'a u32, forward: &bool) -> (&u32, bool) {

        //self.segments.iter().filter(|segment| segment.id.eq(segment_id)).next().id
        //.find instead of .filter will be faster
        let counter = 0;
        for segment in &self.segments {
            if segment_id == &segment.id && *forward {
                if segment == self.segments.last().unwrap() && self.looping == true {
                    return (&self.segments[counter + 1].id, false);
                } if segment == self.segments.last().unwrap() {
                    return (&self.segments[counter - 1].id, true);
                } else {
                    return (&self.segments[counter + 1].id, false);
                }
            }
            else if segment_id == &segment.id && *forward == false {
                if segment == self.segments.first().unwrap() && self.looping == true {
                    return (&self.segments[counter + 1].id, false);
                } if segment == self.segments.first().unwrap() {
                    return (&self.segments[counter + 1].id, true);
                } else {
                    return (&self.segments[counter - 1].id, false);
                }
            }
            else { continue }
        }
        (segment_id, false)
    }
    fn add_segment_start<'b: 'a>(&'b mut self, segment_id: &mut u32, start_station: u32) -> &route::Route {
        *segment_id += 1;
        let old_first_station = self.segments.first().unwrap().start_station;
        let new_segment = route::Route::new(segment_id, &self.name, start_station, old_first_station);
        self.segments.insert(0, new_segment);
        self.start_segment = Some(self.segments.first().unwrap().id);
        self.segments.first().unwrap() 
    }
    fn add_segment_middle<'b: 'a>(&'b mut self, segment_id: &mut u32, start_station: u32, end_station: u32) -> 
        Option<&route::Route> {
        *segment_id += 1;
        let mut i = 0;
        let mut index = i;
        if self.segments.last().unwrap().end_station == start_station && 
            self.segments.first().unwrap().start_station == end_station {
                let new_segment = route::Route::new(segment_id, &self.name, start_station, end_station);
                self.segments.push(new_segment);
                self.start_segment = None;
                self.end_segment = None;
                self.looping = true;
                return Some(self.segments.last().unwrap());
        }
        for segment in &self.segments {
            i += 1;
            if segment.end_station == start_station { //insert new segment after existing one
                index = i + 1;
                *segment_id -= 1;
                if self.segments.get(index) == None {
                    return Some(self.add_segment_end(segment_id, end_station));
                }
                let new_segment = route::Route::new(segment_id, &self.name, start_station, end_station);
                self.start_segment = Some(end_station);
                self.segments.insert(index, new_segment);
                return Some(self.segments.get(index).unwrap());
            }
        }
        None
    }
    fn merge_segments<'b: 'a>(&'b mut self, common_station: &u32) -> Result<&route::Route, LineError> {
        let counter: usize = 0;
        for segment in &mut self.segments {
            if segment.end_station == *common_station {
                segment.end_station = self.segments.get(counter + 1).unwrap().end_station;
                self.segments.remove(counter + 1);
                if segment == self.segments.last().unwrap() && self.looping == false {
                    self.end_segment = Some(segment.id);
                }
                return Ok(self.segments.get(counter).unwrap());
            }
        }
        return Err(LineError::NoCommonStationError);
    }
    fn remove_segment(&mut self, segment_id: &u32) -> Result<route::Route, LineError> {
        let mut counter: usize = 0;
        for segment in &mut self.segments {
            if segment_id == &segment.id {
                if segment == self.segments.first().unwrap() && self.looping == true {
                    self.segments.last_mut().unwrap().end_station = self.segments.get(1).unwrap().start_station;
                    self.segments.get_mut(1).unwrap().start_station = self.segments.last().unwrap().end_station;
                    return Ok(self.segments.remove(0));
                } else if segment == self.segments.first().unwrap() {
                    self.start_segment = Some(self.segments[1].id);
                    let removed_segment = self.segments.remove(0);
                    return Ok(removed_segment);
                } else if segment == self.segments.last().unwrap() && self.looping == true {
                    let removed_segment = self.segments.pop();
                    self.segments.first_mut().unwrap().start_station = self.segments.last().unwrap().end_station;
                    self.segments.last_mut().unwrap().end_station = self.segments.first().unwrap().start_station;
                    return Ok(removed_segment.unwrap());
                } else if *segment == *self.segments.last().unwrap() {
                    let removed_segment = self.segments.pop();
                    self.end_segment == Some(self.segments.last().unwrap().id);
                    return Ok(removed_segment.unwrap());
                } else {
                    self.segments.get_mut(counter - 1).unwrap().end_station = 
                        self.segments.get(counter + 1).unwrap().start_station;
                    self.segments.get_mut(counter + 1).unwrap().start_station = 
                        self.segments.get(counter - 1).unwrap().end_station;
                    let removed_segment = self.segments.remove(counter);
                    return Ok(removed_segment);
                }
            }
            else { counter += 1 }
        }
        Err(LineError::RemoveSegmentError)
    }
}
