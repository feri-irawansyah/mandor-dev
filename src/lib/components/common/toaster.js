import { toast } from "@zerodevx/svelte-toast";

// @ts-ignore
const toaster = {
    // @ts-ignore
    success: (message) => {
        toast.push("Success!", {
            theme: {
                "--toastColor": "mintcream",
                "--toastBackground": "rgba(72,187,120,0.9)",
                "--toastBarBackground": "#2F855A",
                "--toastBarHeight": 0,
            },
        });
    },
    // @ts-ignore
    error: (message) => {
        toast.push("Error: " + message, {
            theme: {
                "--toastColor": "mintcream",
                "--toastBackground": "rgba(237, 100, 166, 0.9)",
                "--toastBarBackground": "#C53030",
                "--toastBarHeight": 0,
            },
        });
    },
};

export default toaster;
