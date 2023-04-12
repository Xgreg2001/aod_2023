#constrained shortest path problem
param n, integer, >= 2; # the number of nodes

set V:={1..n}; # the set of nodes
set A within V cross V; # the set of arcs

param c{(i,j) in A} > 0; # cij the cost of arc (i,j)
param t{(i,j) in A} >= 0; # tij the time of arc (i,j)
param s in V, default 1; # source s
param d in V, != s, default n; # destination d
param T >= 0; # the time limit

var x{(i,j) in A}, >= 0, <= 1 integer;

/* x[i,j] =1 if arc belongs to the shortest path, 0 otherwise*/
minimize Cost: sum{(i,j) in A} c[i,j]*x[i,j];

# flow conservation
s.t. node{i in V}:
sum{(j,i) in A} x[j,i] + (if i = s then 1)= sum{(i,j) in A} x[i,j] + (if i = d then 1);
# time limit
s.t. time{i in V}:
sum{(j,i) in A} t[j,i]*x[j,i] <= T;

solve;

display x;

data;
param n:= 10;

set A:= (1,2) (1,3) (2,4) (2,5) (3,6) (3,7) (4,8) (4,9) (5,8) (5,9) (6,10) (7,10) (8,10) (9,10);

param c:= [1,2] 0.9 [1,3] 0.3 [2,4] 0.5 [2,5] 0.4 [3,6] 0.7 [3,7] 0.2 [4,8] 1 [4,9] 0.8 [5,8] 0.6 [5,9] 2 [6,10] 0.1 [7,10] 0.6 [8,10] 0.3 [9,10] 1;
# set param t equal to 1/c
param t:= [1,2] 1.11 [1,3] 3.33 [2,4] 2 [2,5] 2.5 [3,6] 1.43 [3,7] 5 [4,8] 1 [4,9] 1.25 [5,8] 1.67 [5,9] 0.5 [6,10] 10 [7,10] 1.67 [8,10] 3.33 [9,10] 1;

param T:= 2;

end;