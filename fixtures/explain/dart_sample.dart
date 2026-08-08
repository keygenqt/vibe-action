abstract class Animal {
  String get name;
  String makeSound();
}

class Dog extends Animal {
  @override
  final String name;

  Dog(this.name);

  @override
  String makeSound() => 'Woof';
}

class AnimalShelter {
  final List<Animal> animals = [];

  void admit(Animal animal) {
    animals.add(animal);
  }

  List<String> allSounds() {
    return animals.map((a) => a.makeSound()).toList();
  }
}
