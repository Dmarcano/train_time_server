use gtfs_structures::Stop;

pub trait GtfsQueries {
    /***
     * Get all the stations for a transit system
     */
    fn get_parent_stations(&self) -> Vec<Stop>;
}

const TRANSITER_DEMO_URL: &'static str = "  ";

pub struct NYCTransitterQueries {}

impl GtfsQueries for NYCTransitterQueries {
    fn get_parent_stations(&self) -> Vec<Stop> {
        todo!()
    }
}
