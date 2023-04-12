
set Products;
set Machines;

param max_time;
param price{Products} >= 0;
param operation_cost{Machines} >= 0;
param material_cost{Products} >= 0;
param max_demand{Products} >= 0;
param operation_time{Products,Machines} >= 0;

var x{Products,Machines} >= 0 integer; # kg

maximize profit: 
    sum{p in Products, m in Machines} x[p,m] * (price[p] - operation_cost[m] - material_cost[p]);

s.t. Demand_Constraint{p in Products}:
    sum{m in Machines} x[p,m] <= max_demand[p];

s.t. Time_Constraint{m in Machines}:
    sum{p in Products} x[p,m] * operation_time[p,m] <= max_time;

solve;

display x;
display sum{p in Products, m in Machines} x[p,m] * (price[p] - operation_cost[m] - material_cost[p]);
display{m in Machines} sum{p in Products} x[p,m] * operation_time[p,m];

data;

set Products:= P1 P2 P3 P4;
set Machines:= M1 M2 M3;

# minutes
param max_time:= 3600;

# minutes per kg
param operation_time: M1 M2 M3 :=
P1 5 10 6
P2 3 6 4
P3 4 5 3
P4 4 2 1;

param price:= P1 9 P2 7 P3 6 P4 5; # USD per kg
param operation_cost:= M1 0.03333333333 M2 0.03333333333 M3 0.05; # USD per minute
param material_cost:= P1 4 P2 1 P3 1 P4 1; # USD per kg
param max_demand:= P1 400 P2 100 P3 150 P4 500; # kg

end;
