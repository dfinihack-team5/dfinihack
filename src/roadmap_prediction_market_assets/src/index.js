import { roadmap_prediction_market } from "../../declarations/roadmap_prediction_market";

document.getElementById("clickMeBtn").addEventListener("click", async () => {
  const name = document.getElementById("name").value.toString();
  // Interact with roadmap_prediction_market actor, calling the greet method
  const greeting = await roadmap_prediction_market.greet(name);

  document.getElementById("greeting").innerText = greeting;
});
