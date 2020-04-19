use super::{Circle, Line, Rect, Vec2};

impl Rect {
    pub fn contains_vec2(&self, v0: &Vec2) -> bool {
        v0.x >= self.left() && v0.x <= self.right() && v0.y >= self.top() && v0.y <= self.bottom()
    }

    pub fn contains_rect(&self, r1: &Rect) -> bool {
        self.contains_vec2(&r1.top_left())
            && self.contains_vec2(&r1.top_right())
            && self.contains_vec2(&r1.bottom_left())
            && self.contains_vec2(&r1.bottom_right())
    }

    pub fn intersects_rect(&self, r1: &Rect) -> bool {
        self.contains_vec2(&r1.top_left())
            || self.contains_vec2(&r1.top_right())
            || self.contains_vec2(&r1.bottom_left())
            || self.contains_vec2(&r1.bottom_right())
    }

    pub fn intersect_rect(&self, r1: &Rect) -> Option<Vec<Vec2>> {
        if !self.intersects_rect(r1) {
            return None;
        }

        let mut intersections: Vec<Vec2> = Vec::new();

        if self.contains_vec2(&r1.top_left()) {
            intersections.push(Vec2 {
                x: self.left() + (r1.left() - self.left()),
                y: self.top() + (r1.top() - self.top()),
            });
        }

        if self.contains_vec2(&r1.top_right()) {
            intersections.push(Vec2 {
                x: self.left() + (r1.right() - self.left()),
                y: self.top() + (r1.top() - self.top()),
            });
        }

        if self.contains_vec2(&r1.bottom_left()) {
            intersections.push(Vec2 {
                x: self.left() + (r1.left() - self.left()),
                y: self.top() + (r1.bottom() - self.top()),
            });
        }

        if self.contains_vec2(&r1.bottom_right()) {
            intersections.push(Vec2 {
                x: self.left() + (r1.right() - self.left()),
                y: self.top() + (r1.bottom() - self.top()),
            });
        }

        Some(intersections)
    }

    pub fn contains_circle(&self, c0: &Circle) -> bool {
        let max_radius = self.top_left().distance(&self.bottom_right()) * 0.5;
        self.contains_vec2(&c0.position) && c0.radius <= max_radius
    }

    pub fn intersects_circle(&self, c0: &Circle) -> bool {
        self.intersects_rect(&c0.get_rect())
    }

    pub fn intersect_circle(&self, c0: &Circle) -> Option<Vec<Vec2>> {
        unimplemented!()
    }
}

impl Circle {
    pub fn contains_vec2(&self, v0: &Vec2) -> bool {
        v0.distance(&self.position) <= self.radius
    }

    pub fn contains_circle(&self, c0: &Circle) -> bool {
        self.contains_vec2(&c0.position) && self.radius > c0.radius * 0.5
    }

    pub fn intersects_circle(&self, c0: &Circle) -> bool {
        let max_distance = self.radius + c0.radius;
        let distance = self.position.distance(&c0.position);

        // Circle of the same size with the same position
        if distance == 0. && self.radius == c0.radius {
            return false;
        }
        // One circle is bigger than the other
        else if distance < f64::abs(self.radius - c0.radius) {
            return false;
        }

        // Circles are close enough?
        distance <= max_distance
    }

    pub fn intersect_circle(&self, c0: &Circle) -> Option<Vec<Vec2>> {
        if !self.intersects_circle(c0) {
            return None;
        }

        let mut intersections: Vec<Vec2> = Vec::new();

        let max_distance = self.radius + c0.radius;
        let distance = self.position.distance(&c0.position);

        if distance == max_distance {
            let mut v = Vec2 {
                x: c0.position.x - self.position.x,
                y: c0.position.y - self.position.y,
            };

            v.normalize();
            v.scale(distance);
            v.x += self.position.x;
            v.y += self.position.y;

            intersections.push(v);
            return Some(intersections);
        }

        let mut a = distance * distance + self.radius * self.radius - c0.radius * c0.radius;
        a = a / (2. * distance);

        let mut middle = Vec2 {
            x: c0.position.x - self.position.x,
            y: c0.position.y - self.position.y,
        };

        middle.normalize();
        middle.scale(a);
        middle.x += self.position.x;
        middle.y += self.position.y;

        let h = f64::sqrt(a * a + c0.radius * c0.radius);

        let mut i = Vec2 {
            x: middle.y,
            y: middle.x,
        };

        i.normalize();
        i.scale(h);
        i.x += middle.x;
        i.y += middle.y;
        intersections.push(i);

        let mut i = Vec2 {
            x: middle.y,
            y: middle.x,
        };

        i.normalize();
        i.scale(-h);
        i.x += middle.x;
        i.y += middle.y;
        intersections.push(i);

        Some(intersections)
    }

    pub fn contains_rect(&self, r0: &Rect) -> bool {
        self.contains_vec2(&r0.top_left())
            && self.contains_vec2(&r0.top_right())
            && self.contains_vec2(&r0.bottom_left())
            && self.contains_vec2(&r0.bottom_right())
    }

    pub fn contains_line(&self, l0: &Line) -> bool {
        self.contains_vec2(&l0.v0) && self.contains_vec2(&l0.v1)
    }

    pub fn interects_line(&self, l0: &Line) -> bool {
        unimplemented!()
    }

    pub fn intersect_line(&self, l0: &Line) -> Option<Vec<Vec2>> {
        unimplemented!()
    }
}

#[test]
fn test_rect() {
    let r0 = Rect {
        position: Vec2 { x: 0., y: 0. },
        size: Vec2 { x: 10., y: 10. },
    };

    let v0 = Vec2 { x: 5., y: 5. };
    println!("{:?}", r0.contains_vec2(&v0));

    let mut r1 = Rect {
        position: Vec2 { x: 5., y: 5. },
        size: Vec2 { x: 5., y: 5. },
    };

    let intersect = r0.intersect_rect(&r1);
    println!("{:?}", intersect);

    r1.position.x += 5.;
    let intersect = r0.intersect_rect(&r1);
    println!("{:?}", intersect);

    r1.position.x += 1.;
    let intersect = r0.intersect_rect(&r1);
    println!("{:?}", intersect);
}

#[test]
fn test_circle() {
    let c0 = Circle {
        position: Vec2 { x: 0., y: 0. },
        radius: 10.,
    };
    let c1 = Circle {
        position: Vec2 { x: 10., y: 0. },
        radius: 10.,
    };

    println!("{:?}", c0.intersect_circle(&c1));
}
