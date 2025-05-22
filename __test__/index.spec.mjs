import test from 'ava'

import { sum, hello, Animal } from '../index.js'

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