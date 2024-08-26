trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

enum AnimalType {
    Dog,
    Cat,
}

enum AnimalEnum {
    DogType(Dog),
    CatType(Cat),
}

impl Animal for AnimalEnum {
    fn speak(&self) {
        match self {
            AnimalEnum::DogType(dog) => dog.speak(),
            AnimalEnum::CatType(cat) => cat.speak(),
        }
    }
}

struct AnimalFactory;

impl AnimalFactory {
    fn create_animal(animal_type: AnimalType) -> AnimalEnum {
        match animal_type {
            AnimalType::Dog => AnimalEnum::Dog(Dog),
            AnimalType::Cat => AnimalEnum::Cat(Cat),
        }
    }
}