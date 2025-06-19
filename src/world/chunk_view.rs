use evenio::prelude::Component;
use std::cmp::{max, min};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, Component)]
pub struct PlayerChunkView {
    center_x: i32,
    center_z: i32,
    view_distance: i32,
}

impl PlayerChunkView {
    pub fn min_x(&self) -> i32 {
        self.center_x - self.view_distance - 1
    }

    pub fn min_z(&self) -> i32 {
        self.center_z - self.view_distance - 1
    }

    pub fn max_x(&self) -> i32 {
        self.center_x + self.view_distance + 1
    }

    pub fn max_z(&self) -> i32 {
        self.center_z + self.view_distance + 1
    }

    pub fn contains(&self, x: i32, z: i32, include_outer: bool) -> bool {
        let distance_x = ((x - self.center_x).abs() - 1).max(0);
        let distance_z = ((z - self.center_z).abs() - 1).max(0);
        let longer = (max(distance_x, distance_z) - include_outer as i32).max(0) as i64;
        let shorter = min(distance_x, distance_z) as i64;

        shorter.pow(2) + longer.pow(2) < self.view_distance.pow(2) as i64
    }

    pub fn is_in_view_distance(&self, x: i32, z: i32) {
        self.contains(x, z, false);
    }
    pub fn each<F: Fn(i32, i32)>(&self, func: F) {
        for x in self.min_x()..=self.max_x() {
            for z in self.min_z()..=self.max_z() {
                if self.contains(x, z, true) {
                    // todo test if this is really necessary
                    func(x, z);
                }
            }
        }
    }
    pub fn difference<F: Fn(i32, i32)>(
        old_view: PlayerChunkView,
        new_view: PlayerChunkView,
        load_chunk: F,
        unload_chunk: F,
    ) {
        if old_view == new_view {
            return;
        }
        if old_view.view_distance > 0
            && new_view.view_distance > 0
            && old_view.min_x() <= new_view.max_x()
            && old_view.max_x() >= new_view.min_x()
            && old_view.min_z() <= new_view.max_z()
            && old_view.max_z() >= new_view.min_z()
        {
            let x1 = min(old_view.min_x(), new_view.min_x());
            let z1 = min(old_view.min_z(), new_view.min_z());
            let x2 = max(old_view.max_x(), new_view.max_x());
            let z2 = max(old_view.max_z(), new_view.max_z());

            for x in x1..=x2 {
                for z in z1..=z2 {
                    let unload = old_view.contains(x, z, true);
                    let load = new_view.contains(x, z, true);
                    if unload != load {
                        if load {
                            load_chunk(x, z);
                        } else {
                            unload_chunk(x, z);
                        }
                    }
                }
            }

            return;
        }

        old_view.each(unload_chunk);
        new_view.each(load_chunk);
    }
}
