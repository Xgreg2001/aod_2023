
set Shifts;
set Districts;

param min_cars{Districts, Shifts} >= 0;
param max_cars{Districts, Shifts} >= 0;

param min_cars_per_shift{Shifts} >= 0;
param min_cars_per_district{Districts} >= 0;

var x{Districts, Shifts} >= 0, integer;

s.t. min_cars_per_shift_constraint{s in Shifts}:
sum{d in Districts} x[d, s] >= min_cars_per_shift[s];

s.t. min_cars_per_district_constraint{d in Districts}:
sum{s in Shifts} x[d, s] >= min_cars_per_district[d];

s.t. max_cars_constraint{d in Districts, s in Shifts}:
x[d, s] <= max_cars[d, s];

s.t. min_cars_constraint{d in Districts, s in Shifts}:
x[d, s] >= min_cars[d, s];

minimize total_cars:
sum{d in Districts, s in Shifts} x[d, s];

solve;

display x;
display sum{d in Districts, s in Shifts} x[d, s];

data;

set Shifts := zmiana1 zmiana2 zmiana3;
set Districts := p1 p2 p3;

param min_cars: zmiana1 zmiana2 zmiana3 :=
p1 2 4 3
p2 3 6 5
p3 5 7 6;

param max_cars: zmiana1 zmiana2 zmiana3 :=
p1 3 7 5
p2 5 7 10
p3 8 12 10;

param min_cars_per_shift:= zmiana1 10 zmiana2 20 zmiana3 18;
param min_cars_per_district:= p1 10 p2 14 p3 13;

end;

