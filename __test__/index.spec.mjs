import test from 'ava'

import { sum, hello, Animal, coolFunction, highOrderFunction, getEnv, Kind, sayA, getSum, Dog, Cat, printPet, sayHi } from '../index.js'

test('sum from native', (t) => {
  t.is(sum(1, 2), 3)
})

test('hello from native', (t) => {
  t.is(hello('world'), 'hello world')
})

test('Animal from native', (t) => {
  const animal = new Animal('dog', 1)
  t.is(animal.name, 'dog')
  t.is(animal.age, 1)
  animal.changeName('cat')
  t.is(animal.name, 'cat')
})

test('coolFunction from native', (t) => {
  t.is(coolFunction(Kind.Dog), 'dog')
  t.is(coolFunction(Kind.Cat), 'cat')
})

test('highOrderFunction from native', (t) => {
  t.is(highOrderFunction(1, (s) => s), 2)
})

test('getEnv from native', (t) => {
  t.is(getEnv('USER'), 'olive')
})

test('sayA from native', (t) => {
  t.is(sayA(true), true)
  t.is(sayA(false), false)

  const err = t.throws(() => sayA(null))
  t.is(err.message, 's is None')
})

test('getSum from native', (t) => {
  t.is(getSum([1, 2, 3]), 6)
})

test('Dog from native', (t) => {
  const dog = new Dog('dog')
  t.is(dog.name, 'dog')
})

test('Cat from native', (t) => {
  const cat = Cat.create('cat')
  t.is(cat.name, 'cat')
})

test('Cat from native set name', (t) => {
  const cat = new Cat('cat')
  t.is(cat.name, 'cat')
  cat.setName('dog')
  t.is(cat.name, 'dog')
  t.is(cat.getName(), 'dog')
})

test('Cat from native say name', (t) => {
  const cat = new Cat('cat')
  t.is(cat.sayName(), 'cat')
})

test('printPet from native', (t) => {
  const pet = { name: 'dog', kind: 1 }
  t.is(printPet(pet), undefined)
})

test('sayHi from native', (t) => {
  t.is(sayHi((a) => {
    console.log(a)
  }), undefined)
})
