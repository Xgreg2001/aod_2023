# load data from output files

import pandas as pd
import os
import matplotlib.pyplot as plt
import seaborn as sns

output_path = "outputs/"


def load_data():
    # creaet empty dataframe
    df = pd.DataFrame(
        columns=[
            "algorithm",
            "nodes",
            "edges",
            "time",
            "min_cost",
            "max_cost",
            "problem",
        ]
    )

    # load data from output files in folder
    for file in os.listdir(output_path):
        with open(output_path + file) as f:
            data = f.readlines()
            (_, nodes, edges, min_cost, max_cost) = data[1].split()
            (_, time) = data[2].split()

            algorithm = file.split("_")[0]
            problem = file.split("_")[1].split(".")[0]

            # add data to dataframe
            df = pd.concat(
                [
                    df,
                    pd.DataFrame(
                        [[algorithm, nodes, edges, time, min_cost, max_cost, problem]],
                        columns=df.columns,
                    ),
                ],
                ignore_index=True,
            )

    return df


# if data.csv exists read from it else creata new data and save it
if not os.path.exists("data.csv"):
    df = pd.read_csv("data.csv", index_col=0)
else:
    df = load_data()
    df.to_csv("data.csv")


# Create directory if it doesn't exist
if not os.path.exists("graphs"):
    os.makedirs("graphs")

problems = df["problem"].unique()

for problem in problems:
    df_problem = df[df["problem"] == problem]

    # Bar plot
    fig, ax = plt.subplots(figsize=(10, 5))
    sns.barplot(x="algorithm", y="time", data=df_problem, ax=ax)
    ax.set_title(
        f"Comparison of Time for Different Algorithms for {problem}", fontsize=16
    )
    ax.set_ylabel("Time", fontsize=14)
    ax.set_xlabel("Algorithm", fontsize=14)
    fig.savefig(f"graphs/{problem}_barplot.pdf")
    plt.close(fig)

    # Line plot for time vs nodes
    fig, ax = plt.subplots(figsize=(10, 5))
    sns.lineplot(
        x="nodes", y="time", data=df_problem, hue="algorithm", marker=".", ax=ax
    )
    ax.set_title(
        f"Comparison of Time for Different Algorithms based on Nodes for {problem}",
        fontsize=16,
    )
    ax.set_ylabel("Time", fontsize=14)
    ax.set_xlabel("Nodes", fontsize=14)
    ax.set_xscale("log")
    fig.savefig(f"graphs/{problem}_nodes_lineplot.pdf")
    plt.close(fig)

    # Line plot for time vs edges
    fig, ax = plt.subplots(figsize=(10, 5))
    sns.lineplot(
        x="edges", y="time", data=df_problem, hue="algorithm", marker=".", ax=ax
    )
    ax.set_title(
        f"Comparison of Time for Different Algorithms based on Edges for {problem}",
        fontsize=16,
    )
    ax.set_ylabel("Time", fontsize=14)
    ax.set_xlabel("Edges", fontsize=14)
    ax.set_xscale("log")
    fig.savefig(f"graphs/{problem}_edges_lineplot.pdf")
    plt.close(fig)

    # Line plot for time vs max_cost
    fig, ax = plt.subplots(figsize=(10, 5))
    sns.lineplot(
        x="max_cost", y="time", data=df_problem, hue="algorithm", marker=".", ax=ax
    )
    ax.set_title(
        f"Comparison of Time for Different Algorithms based on Max Cost for {problem}",
        fontsize=16,
    )
    ax.set_ylabel("Time", fontsize=14)
    ax.set_xlabel("Max Cost", fontsize=14)
    ax.set_xscale("log")
    fig.savefig(f"graphs/{problem}_max_cost_lineplot.pdf")
    plt.close(fig)

for problem in problems:
    df_problem = df[df["problem"] == problem]
    df_problem_sorted = df_problem.sort_values(
        by=["algorithm", "nodes", "edges", "max_cost"]
    )

    latex_code = df_problem_sorted.to_latex(
        index=False
    )  # Convert the DataFrame to LaTeX table

    # Write the LaTeX code to a .tex file
    with open(f"graphs/{problem}_table.tex", "w") as file:
        file.write(latex_code)
