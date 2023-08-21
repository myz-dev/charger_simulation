/**
 * Data structure that is expected by the server on simulation requests.
 */
export class SimInput {
    constructor() {
    }
    arrival_modifier: number = 1;
    station_count: number = 30;
    battery_consumption: number = 18;
    station_power: number = 30;

}

/**
 * Data structure with which the server sends its response.
 */
export class SimOutput {
    constructor() {
    }
    actual_max_kW: number = 0;
    theoretical_max_kW: number = 0;
    concurrency: number = 0;
    consumption_year_kWh: number = 30;

    static produce_mock_output(): SimOutput {
        let output = new SimOutput();
        output.actual_max_kW = 132;
        output.consumption_year_kWh = 147765.75;
        output.theoretical_max_kW = 330;
        return output;
    }
}


function is_SimOutput(obj: any): obj is SimOutput {
    return obj && typeof obj === 'object' &&
        'actual_max_kW' in obj &&
        "theoretical_max_kW" in obj &&
        "concurrency" in obj &&
        "consumption_year_kWh" in obj
}

/**
 * Sends the input parameters to the server and returns the sim output on success.
 * Throws an error if something goes wrong.
 * @param sim_input
 * @returns simulation output
 */
export async function post_sim_request(sim_input: SimInput): Promise<SimOutput> {
    const URL = "/post/sim_input";
    try {
        const response = await fetch(URL,
            { method: "POST", headers: { "Content-Type": "application/json" }, body: JSON.stringify(sim_input) });
        if (!response.ok) {
            throw new Error("Todo: good error handling. Details: " + response.statusText);
        }
        const data = await response.json();
        if (!is_SimOutput(data)) {
            throw new Error("Todo: good error handling. Bad return type.");
        }

        console.log("great, we got the output!");
        return Promise.resolve(data);
    } catch (error) {
        console.log("Todo: Handle errors gracefully.");
        console.log(error);
        return Promise.reject(new Error("Something went wrong! " + error));
    }
}
