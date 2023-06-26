import matplotlib.pyplot as plt
import os
import pandas as pd

folder_name = "data"

df = pd.DataFrame(columns=["algo", "k", "max-flow", "augmenting-paths" "seconds"])

for file_name in os.listdir(folder_name):
    contents = open(os.path.join(folder_name, file_name)).read()
    algo = file_name.split("_")[0]

    # extract data from file
    # example:
    # k: 1
    # Max flow: 2
    # Augmenting paths: 1
    # Elapsed: 0.000s

    lines = contents.split("\n")
    k = int(lines[0].split(":")[1])
    max_flow = int(lines[1].split(":")[1])
    augmenting_paths = int(lines[2].split(":")[1])
    seconds = float(lines[3].split(":")[1][:-1])

    # add data to dataframe containing all data
    new_row = {"algo": algo, "k": k, "max-flow": max_flow, "augmenting-paths": augmenting_paths, "seconds": seconds}
    df = pd.concat([df, pd.DataFrame([new_row])], ignore_index=True)

# merge columns with the same k and algo by calculating average time and paths
df = df.groupby(["algo", "k"]).mean().reset_index()


# plot data comparing two algorithms "dinic" and "edmonds-karp"

df_dinic = df[df["algo"] == "dinic"]
df_edmonds_karp = df[df["algo"] == "edmonds-karp"]

plt.plot(df_dinic["k"], df_dinic["seconds"], label="dinic")
plt.plot(df_edmonds_karp["k"], df_edmonds_karp["seconds"], label="edmonds-karp")
plt.xlabel("k")
plt.ylabel("seconds")
plt.legend()
plt.savefig("plot.pdf")
plt.clf()

plt.plot(df_dinic["k"], df_dinic["augmenting-paths"], label="dinic")
plt.plot(df_edmonds_karp["k"], df_edmonds_karp["augmenting-paths"], label="edmonds-karp")
plt.xlabel("k")
plt.ylabel("augmenting paths")
plt.legend()
plt.savefig("plot2.pdf")
plt.clf()




