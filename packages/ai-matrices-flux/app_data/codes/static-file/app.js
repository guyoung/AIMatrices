document.getElementById("generate-form").addEventListener("submit", async (e) => {
    e.preventDefault();
    const prompt = document.getElementById("prompt").value;
    const translateToEnglish = document.getElementById("translate-checkbox").checked;

    // Show loading spinner
    document.getElementById("loading-spinner").style.display = "block";
    document.getElementById("generated-image").style.display = "none";

    try {
        const response = await fetch("/service/flux", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                preset: "Flux1Dev",
                prompt: prompt,
                translateToEnglish: translateToEnglish
            }),
        });

        if (response.ok) {
            const data = await response.text();

            document.getElementById("generated-image").src = data;
            document.getElementById("generated-image").style.display = "block";
        } else {
            alert("Failed to generate image");
        }
    } catch (error) {
        console.error("Error:", error);
        alert("An error occurred while generating the image");
    } finally {
        // Hide loading spinner
        document.getElementById("loading-spinner").style.display = "none";
    }
});