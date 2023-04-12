param n >=0 integer; 
param m >=0 integer;
param k >=0 integer; # camera range

set N := {1..n};
set M := {1..m};

set Grid := N cross M;

param conteiners{(i,j) in Grid} >=0 <=1 integer;

var cameras{(i,j) in Grid} >=0 <=1 integer;

minimize total_cameras: sum{(i,j) in Grid} cameras[i,j];

s.t. camera_cant_be_in_conteiners{(i,j) in Grid}:
    cameras[i,j] <= 1 - conteiners[i,j];

s.t. conteiners_must_be_watched{(i,j) in Grid}:
    sum{x in 1..n : abs(x - i) <= k} cameras[x,j] + sum{y in 1..n : abs(y - j) <= k} cameras[i,y] >= conteiners[i,j];

solve;

display cameras;
display sum{(i,j) in Grid} cameras[i,j];

data;
param n:= 5;
param m:= 5;

param k:= 2;

param conteiners: 1 2 3 4 5 :=
1 1 0 0 0 1
2 0 0 0 1 0
3 0 1 0 0 0
4 0 0 1 0 1
5 1 0 1 0 0;

end;