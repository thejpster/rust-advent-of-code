use failure::Error;

#[derive(Debug, Clone)]
struct Particle {
    idx: usize,
    p: Vec3,
    v: Vec3,
    a: Vec3,
}

#[derive(Copy, Clone, PartialEq, Eq, Ord)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl ::std::cmp::PartialOrd for Vec3 {
    fn partial_cmp(&self, rhs: &Vec3) -> Option<::std::cmp::Ordering> {
        if self.x == rhs.x {
            if self.y == rhs.y {
                Some(self.z.cmp(&rhs.z))
            } else {
                Some(self.y.cmp(&rhs.y))
            }
        } else {
            Some(self.x.cmp(&rhs.x))
        }
    }
}

impl ::std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ::std::fmt::Debug for Vec3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(fmt, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Particle {
    fn distance(&self) -> i64 {
        self.p.distance()
    }

    fn parse(idx: usize, input: &str) -> Result<Particle, Error> {
        let mut parts = input.split(", ");
        let p = parts.next().unwrap();
        let v = parts.next().unwrap();
        let a = parts.next().unwrap();
        Ok(Particle {
            idx: idx,
            p: Vec3::parse(&p[3..p.len() - 1])?,
            v: Vec3::parse(&v[3..v.len() - 1])?,
            a: Vec3::parse(&a[3..a.len() - 1])?,
        })
    }

    fn step(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }
}

impl Vec3 {
    fn distance(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn parse(input: &str) -> Result<Vec3, Error> {
        let parts: Vec<&str> = input.split(',').collect();
        Ok(Vec3 {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            z: parts[2].parse()?,
        })
    }
}

pub fn run(contents: &[Vec<String>]) {
    let particles_orig: Result<Vec<Particle>, Error> = contents[0]
        .iter()
        .enumerate()
        .map(|(i, l)| Particle::parse(i, l))
        .collect();
    let particles_orig = particles_orig.expect("Couldn't parse input");

    let steps = 2_000_000;
    let mut particles1 = particles_orig.clone();
    for _ in 0..steps {
        for particle in &mut particles1 {
            particle.step();
        }
    }
    particles1.sort_by_key(|p| p.distance());
    println!(
        "Closest particles after {} steps: {:?}",
        steps,
        &particles1[0..3]
    );

    let mut particles2 = particles_orig;
    let mut last_count = 0;
    for i in 0..1_000 {
        for particle in &mut particles2 {
            particle.step();
        }
        let mut duplicates = Vec::new();
        for i in 0..particles2.len() {
            for j in i + 1..particles2.len() {
                if particles2[i].p == particles2[j].p {
                    duplicates.push(i);
                    duplicates.push(j);
                }
            }
        }
        duplicates.sort();
        duplicates.dedup();
        duplicates.reverse();
        for idx in duplicates {
            particles2.remove(idx);
        }
        if particles2.len() != last_count {
            println!("Particles left after {} steps: {}", i, particles2.len());
            last_count = particles2.len();
        }
    }
}
