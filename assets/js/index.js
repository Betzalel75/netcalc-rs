function start() {
  const lignes = [
    document.querySelectorAll('#ligne1 .bit'),
    document.querySelectorAll('#ligne2 .bit'),
    document.querySelectorAll('#ligne3 .bit'),
  ];

  let col = lignes[0].length - 1; // Commence à droite
  let delay = 0;
  const interval = 1000;

  function animCol(i) {
    for (let ligne of lignes) {
      const bit = ligne[i]; // ici, ligne est une NodeList de .bit
      setTimeout(() => {
        if (bit && bit.classList) {
          bit.classList.add('active');
          setTimeout(() => {
            bit.classList.remove('active');
          }, interval - 50);
        }
      }, delay);
      delay += interval;
    }
  }

  function run() {
    col = lignes[0].length - 1; // Réinitialiser la colonne à chaque exécution
    delay = 0; // Réinitialiser le délai
    for (; col >= 0; col--) {
      animCol(col);
    }

    // Une fois l'animation terminée, la relancer après un délai
    const delaiTotalAnimation = lignes[0].length * lignes.length * interval;
    setTimeout(run, delaiTotalAnimation + 500);
  }

  run(); // Démarrer la première animation
}

start();