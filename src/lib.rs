#![allow(unused)]

#[derive(Debug, PartialEq)]
pub struct Car<'a> {
    name: &'a str,
    brand: &'a str,
    is_electric: bool,
    cost: u32,
}

impl<'a> Car<'a> {
    fn sample() -> Vec<Car<'a>> {
        vec![
            Car { name: "Model 3", brand: "Tesla",   cost: 60000, is_electric: true,  },
            Car { name: "350z",    brand: "Nissan",  cost: 20000, is_electric: false, },
            Car { name: "86",      brand: "Toyota",  cost: 45000, is_electric: false, },
            Car { name: "i30",     brand: "Hyundai", cost: 10000, is_electric: false, },
            Car { name: "Model 3", brand: "Tesla",   cost: 30000, is_electric: true,  },
        ]
    }
}


/// ## 1. return an array of only electric cars.
/// 
/// Typescript version
/// ```typescript
/// function only_electric(cars: Car[]): Car[] {
///     return cars.filter(car => car.isElectric);
/// }
/// ```
pub fn only_electric<'a> (cars: &'a [Car]) -> Vec<&'a Car<'a>> {
    cars.iter().filter(|car| car.is_electric).collect()
}

/// ## 2. return the cost of all cars together.
/// 
/// Typescript version
/// ```typescript
/// function total_cost(cars: Car[]): number {
///     return cars.reduce((acc, car) => acc + car.cost, 0);
/// }
/// ```
pub fn total_cost(cars: &[Car]) -> u32 {
    cars.iter().fold(0, |acc, car| acc + car.cost)
}

/// ## 3. return unique car brands.
/// 
/// Typescript version
/// ```typescript
/// function unique_brands(cars: Car[]): string[] {
///     let brands: string[] = cars
///         .map((car) => car.brand)
///         .reduce((acc, car) => {
///             if (!acc.includes(car)) {
///                 acc.push(car);
///             }
///             return acc;
///         }, [] as string[]);
///
///     return brands;
/// }
/// ```
/// 
/// Typescript version 2
/// ```typescript
/// function uniqueBrands(cars: Car[]): string[] {
///     const brands = cars
///         .map((car) => car.brannd)
///         .filter((brand, index, arr) => arr. findIndex((b) => brand == b) == index);
///
///     return brands;
/// }
pub fn unique_brands<'a> (cars: &'a [Car]) -> Vec<&'a str> {
    cars
        .iter()
        .map(|car| car.brand)
        .fold(vec![], |mut acc, brand| {
            if !acc.contains(&brand) {
                acc.push(brand);
            }
            acc
        })
}

pub fn unique_brands2<'a> (cars: &'a [Car]) -> Vec<&'a str> {
    cars
        .iter()
        .map(|car| car.brand)
        .enumerate()
        .filter(|(i, brand)| {
            cars.iter().position(|car| car.brand == *brand) == Some(*i)
        })
        .map(|(i, brand)| brand)
        .collect()
}

pub fn unique_brands3<'a> (cars: &'a [Car]) -> Vec<&'a str> {
    let mut brands: Vec<&str> = cars.iter().map(|car| car.brand).collect();
    brands.sort();
    brands.dedup();
    brands
}

/// ## 4. are all cars electric?
/// 
/// Typescript version
/// ```typescript
/// function allElectric(cars: Car[]): boolean {
///     return cars.every(car => car.isElectric);
/// }
/// ```
pub fn all_electric<'a> (cars: &'a [Car]) -> bool {
    cars.iter().all(|car| car.is_electric)
}


/// ## 5. is any car electric?
/// 
/// Typescript version
/// ```typescript
/// function cxuIuElektra(cars: Car[]): boolean {
///     return cars.some(car => car.isElectric);
/// }
/// ```
pub fn any_electric<'a> (cars: &'a [Car]) -> bool {
    cars.iter().any(|car| car.is_electric)
}

/// ## 6. find the last non-electric car of the array
/// 
/// Typescript version
/// ```typescript
/// function last_non_electric(cars: Car[]): Car | undefined {
///     return cars.reverse().find(car => !car.isElectric);
/// }
/// ```
pub fn last_non_electric<'a> (cars: &'a [Car]) -> Option<&'a Car<'a>> {
    cars.iter().rfind(|car| !car.is_electric)
}

/// ## 7. find the most expensive car in the array
/// 
/// Typescript version
/// ```typescript
/// function plejMultekosta(cars: Car[]): Car {
///     return cars.sort((a, b) => b.cost - a.cost)[0];
/// }
/// ```
pub fn most_expensive<'a> (cars: &'a [Car]) -> Option<&'a Car<'a>> {
    cars.iter().reduce(|most_expensive, car| {
        if car.cost > most_expensive.cost { return car }
        most_expensive
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    fn sample_answer<'a> (cars: &'a Vec<Car>, indices: Vec<usize>) -> Vec<&'a Car<'a>> {
        let mut answer = vec![];
        for i in indices {
            answer.push(&cars[i]);
        }
        answer
    }

    #[test]
    fn test_only_electric() {
        let cars = Car::sample();
        let answer = sample_answer(&cars, vec![0, 4]);

        assert_eq!(only_electric(&cars), answer);
    }

    #[test]
    fn test_total_cost() {
        let cars = Car::sample();
        let answer = 165_000;

        assert_eq!(total_cost(&cars), answer);
    }

    #[test]
    fn test_unique_brands() {
        let cars = Car::sample();
        let answer = vec!["Tesla", "Nissan", "Toyota", "Hyundai"];

        assert_eq!(unique_brands(&cars), answer);
    }

    #[test]
    fn test_unique_brands2() {
        let cars = Car::sample();
        let answer = vec!["Tesla", "Nissan", "Toyota", "Hyundai"];

        assert_eq!(unique_brands2(&cars), answer);
    }

    #[test]
    fn test_unique_brands3() {
        let cars = Car::sample();
        let mut answer = vec!["Tesla", "Nissan", "Toyota", "Hyundai"];
        answer.sort();

        assert_eq!(unique_brands3(&cars), answer);
    }

    #[test]
    fn test_all_electric() {
        let cars = Car::sample();
        let answer = false;

        assert_eq!(all_electric(&cars), answer);
    }

    #[test]
    fn test_any_electric() {
        let cars = Car::sample();
        let answer = true;

        assert_eq!(any_electric(&cars), answer);
    }

    #[test]
    fn test_last_non_electric() {
        let cars = Car::sample();
        let answer = cars.get(3);

        assert_eq!(last_non_electric(&cars), answer);
    }

    #[test]
    fn test_most_expensive() {
        let cars = Car::sample();
        let answer = cars.get(0);

        assert_eq!(most_expensive(&cars), answer);
    }
}
