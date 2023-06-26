import os
import pandas as pd

folder_name = "data"

df = pd.DataFrame(columns=["k", "i", "matchings", "milis"])

for file_name in os.listdir(folder_name):
    contents = open(os.path.join(folder_name, file_name)).read()
    k = int(file_name.split("-")[0])
    i = int(file_name.split("-")[1])

    matchings = int(contents.split("Matching: ")[1].split("\n")[0])
    milis=int(contents.split("Time: ")[1].split("\n")[0])

    # add data to dataframe containing all data
    new_row = {"k": k, "i": i, "matchings": matchings, "milis": milis}
    df = pd.concat([df, pd.DataFrame([new_row])], ignore_index=True)

# merge columns with the same k and i by calculating average time and matchings
df = df.groupby(["k", "i"]).mean().reset_index()

# plot data
import matplotlib.pyplot as plt

# crate a plot of time depending on k (calculate average time for each k) sorted by k
df.groupby("k").mean()["milis"].plot()
plt.xlabel("k")
plt.ylabel("time [ms]")
plt.savefig("plot_time.pdf")
plt.clf()

# crate a plot of matchings depending on k (calculate average matchings for each k) sorted by k
df.groupby("k").mean()["matchings"].plot()
plt.xlabel("k")
plt.ylabel("matchings")
plt.savefig("plot_matchings.pdf")
plt.clf()

# crate a plot of time depending on i (calculate average time for each i) sorted by i
df.groupby("i").mean()["milis"].plot()
plt.xlabel("i")
plt.ylabel("time [ms]")
plt.savefig("plot_time_i.pdf")
plt.clf()

# crate a plot of matchings depending on i (calculate average matchings for each i) sorted by i
df.groupby("i").mean()["matchings"].plot()
plt.xlabel("i")
plt.ylabel("matchings")
plt.savefig("plot_matchings_i.pdf")



