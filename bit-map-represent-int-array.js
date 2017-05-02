//from Programming Pearls
//to represent a unique int array { 1, 3, 5, 8 }, a bit string is ENOUGH: 010101001, every 1 stands for an integer n at pos n, from left to right.
//this usually save space (ascii '010101001' bytes vs 4 integer bytes), except for { 1, 1000000 }.
//so all depend on sparse likelihood