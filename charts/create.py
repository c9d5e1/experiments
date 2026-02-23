import json
import matplotlib.pyplot as plt
from matplotlib.ticker import MaxNLocator
import numpy as np
import os

REPORTS_FOLDER = "reports"
OUTPUT_FOLDER = "charts"

for filename in os.listdir(REPORTS_FOLDER):
    filepath = os.path.join(REPORTS_FOLDER, filename)
    with open(filepath, 'r') as f:
        full_data = json.load(f)
        data = full_data['run_reports']

    # Extract values
    x_values = list(range(len(data)))
    total_requests = [item['total_requests'] for item in data]
    avg_elapsed = [item['average_elapsed_millis'] for item in data]
    total_errors = [item['total_errors'] for item in data]

    # Create plot
    plt.figure(figsize=(12, 7))

    plt.plot(x_values, total_requests, marker='o', linestyle='-', color='blue', label='Total Requests')
    plt.plot(x_values, avg_elapsed, marker='o', linestyle='-', color='green', label='Average Elapsed (ms)')
    plt.plot(x_values, total_errors, marker='o', linestyle='-', color='red', label='Total Errors')

    # Add trendlines
    def add_trendline(x, y, color):
        z = np.polyfit(x, y, 1)
        p = np.poly1d(z)
        plt.plot(x, p(x), linestyle='--', color=color)

    add_trendline(x_values, total_requests, 'blue')
    add_trendline(x_values, avg_elapsed, 'green')
    add_trendline(x_values, total_errors, 'red')

    # Labels and styling
    plt.xlabel('Run Index')
    plt.ylabel('Values')
    plt.title(full_data['configuration']['label'])
    plt.legend()
    plt.grid(True)

    plt.gca().xaxis.set_major_locator(MaxNLocator(integer=True))

    # Save image
    plt.savefig(f'{OUTPUT_FOLDER}/{full_data['configuration']['label']}.png', dpi=300)