
set Company;
set Airport;

param supply{Company} >= 0;
param demand{Airport} >= 0;
param cost{Airport, Company} >= 0;

var x{Airport, Company} >= 0;

minimize Total_Cost: sum {c in Company, a in Airport} cost[a,c]*x[a,c];

s.t. Supply_Constraint{c in Company}: sum {a in Airport} x[a,c] <= supply[c];
s.t. Demand_Constraint{a in Airport}: sum {c in Company} x[a,c] >= demand[a];

solve;

display sum {c in Company, a in Airport} cost[a,c]*x[a,c];
display x;


data;
# Definitions of the sets
set Company:= Firma1 Firma2 Firma3;
set Airport:= Lotnisko1 Lotnisko2 Lotnisko3 Lotnisko4;

# Initialization of the parameters
param supply:= Firma1 275000 Firma2 550000 Firma3 660000;
param demand:= Lotnisko1 110000 Lotnisko2 220000 Lotnisko3 330000 Lotnisko4 440000;
param cost: Firma1 Firma2 Firma3:=
Lotnisko1 10 7 8
Lotnisko2 10 11 14
Lotnisko3 9 12 4
Lotnisko4 11 13 9;

end;
