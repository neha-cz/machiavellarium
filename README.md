# marvarium

studying how emotional introspection and multi-agent social interactions in LLMs with unfrozen weights in a simulated environment can induce/enhance learning. heavily inspired and motivated by the emotion machine by marvin minsky

the idea: 
1) different fine-tuned agents interact in a simulated environment, involving attachment, pain, and social pressures
2) determine the moment-to-moment signal of extracted emotion-concept vectors from each interaction
3) following the free energy principle (fep), calculate valence and mood: valence is the smoothed rate of change of the relevant emotion activations and mood is a slower exponential average of valence
4) if the original interaction involved "surprise", as defined by the fep, perform the following: 
5) mood = manic; loosen the kl constraint; mood = depressive, increase the kl constraint
6) ask the agent to verbally reflect on the interaction, which becomes the concrete training signal gated by step 5
7) update the weights accordingly 
  
the simulated world will primarily be scaffolded in rust, with agent implemenations in python 

### references:

- Minsky, Marvin. The Emotion Machine: Commonsense Thinking, Artificial Intelligence, and the Future of the Human Mind. (2006)
- Friston, Karl. The free-energy principle: a unified brain theory? (2010)
- Joffily, Matteus & Coricelli, Giorgio. Emotional Valence and the Free-Energy Principle (2013)
- Park, Joon Sung, et al. Generative Agents: Interactive Simulacra of Human Behavior (2023)
- Park, Joon Sung, et al. LLM Agents Grounded in Self-Reports Enable General-Purpose Simulation of Individuals (2024)
- Jaques, Natasha. Social Influence as Intrinsic Motivation for Multi-Agent Deep Reinforcement Learning. (2018)
- Lindsey, Jack. Emergent Introspective Awareness in Large Language Models. (2026)
