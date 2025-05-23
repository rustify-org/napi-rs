import test from 'ava'

import { sum, hello, Animal, coolFunction, highOrderFunction, getEnv, Kind } from '../index.js'

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