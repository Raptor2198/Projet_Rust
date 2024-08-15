# Number Game

"Number Game" est un jeu multijoueur en réseau où plusieurs joueurs devinent un nombre secret en recevant des indices "C'est plus" ou "C'est moins". Le jeu propose trois niveaux de difficulté : facile, moyen et difficile, que les joueurs peuvent voter avant de commencer à jouer.y

Démarrage : 
- Pour lancer le serveur:
    cargo run --bin server

-Pour lancer un client(plusieurs clients peuvent etre lancées avec plusieurs terminales):
    cargo run --bin client


Fonctionnalités actuelles :
-Multijoueur avec un serveur multithread
-Choix du niveau de difficulté par vote (facile, moyen, difficile)
-Indices pour aider à deviner le nombre secret
-Phases de jeu claires : phase d'identification, phase de vote(un countdown inclus), phase de jeu(phase où on fait les guess)
-Tests unitaires pour les principales fonctionnalités.

Avancement:
Plusieurs fonctionnalités ont déja été développés et je pense à ajouter un tableau de score dans le futur et une liste de joueurs(quelques-uns des warnings sont dues à des améliorations pas fini mais encore en cours de développement). Un système de récompenses pourraient également être mis en places. player.rs a été créer pour encapsuler la logique des joeurs mais n'est pas encore utilisé dans les uatres parties du code pour l'instant. Dans les prochaines mise à jours, il sera utilisé.


Architecture du code : 
                                                       ** Structure des fichiers
client.rs : Contient la logique du client, y compris la gestion de la connexion au serveur, l'envoi et la réception des messages.

game.rs : Contient la logique du jeu, y compris la gestion des joueurs, des devinettes, des votes de difficulté, etc.

main.rs : Point d'entrée du serveur, lance le serveur et gère les connexions des différents clients.

player.rs : Contient la structure et les méthodes pour gérer les joueurs.

server.rs : Contient la logique du serveur, y compris la gestion des clients et la diffusion des messages.

util.rs : Contient les structures et les fonctions utilitaires pour sérialiser/désérialiser les messages et diffuser les messages aux clients.

Cargo.toml : Fichier de configuration des dépendancesdu projet.



                                                      Explication et justification des choix effectués
Pourquoi Rust et pas un autre language pour ce projet ?
Pour sa gestion de la concurrence qui permet d'assurer une communication fluide et efficace entre le serveur et les clients.

Communication réseau
La communication réseau est gérée à l'aide des sockets TCP. Chaque client se connecte au serveur et communique via des messages sérialisés en utilisant la bibliothèque bincode. Les messages échangés entre le client et le serveur sont définis dans le module util.rs et incluent des types de messages pour les devinettes, les votes de difficulté et les notifications de début et de fin de jeu.

Phases de jeu
Le jeu est divisé en trois phases :
- Identification : Les joueurs se connectent et s'identifient par un nom.
- Vote : Les joueurs votent pour le niveau de difficulté durant un countdown de 20 secondes.
- Jeu : Les joueurs devinent le nombre secret et recoivent un indice pour chaque guess qu'ils font.
Cette structure a été choisi parce qu'elle est logique et permet une expérience de jeu plus fluide.

Concurrence:
- Le projet utilise le multithreading pour gérer plusieurs connexions clients simultanément et pour gérer le compte à rebours du vote de difficulté. En effet, nous avons un serveur multithread.
- La gestion de la concurrence est réalisée en utilisant les Arc et Mutex de Rust pour partager en toute sécurité l'état du jeu et les clients entre plusieurs threads. La bibliothèque crossbeam est utilisée pour gérer les threads et faciliter la communication entre eux.

Explication plus détaillé de l'utilisation du Multithreading dans le projet :

- Gestion des Connexions Clients :
Lorsqu'un nouveau client se connecte, un nouveau thread est créé pour gérer la communication avec ce client. Cela permet au serveur de continuer à accepter d'autres connexions pendant que les clients déja connectés interagissent avec le jeu. Le code pertinent se trouve dans server.rs, où chaque nouvelle connexion crée un nouveau thread via scope.spawn.
Chaque thread individuel s'occupe de la logique de jeu pour chaque joueur.Chaque thread fonctionne indépendamment, ce qui signifie que chaque joueur peut jouer à son propre rythme sans affecter les autres. Si un joueur fait une pause ou réfléchit longtemps avant de faire un guess, les autres joueurs peuvent continuer à jouer sans interruption.

- Compte à Rebours(Countdown) du Vote de Difficulté :
Un thread séparé est utilisé pour gérer le compte à rebours de la phase de vote. Cela permet de vérifier régulièrement si le temps de vote est écoulé et de démarrer la phase de jeu. Ce mécanisme est implémenté dans server.rs, dans la boucle principale du thread de gestion du jeu.


Pourquoi le choix d'utilisation du multithreading pour le projet ?  ==> Meilleure performance, excellente réactivité,  et expérience utilisateur fluide.
- Meilleure performance: 
Le multithreading permet au serveur de traiter plusieurs connexions en même temps. Cela signifie que lorsque plusieurs joueurs se connectent, le serveur peut répondre à chacun d'eux sans délai.

- Excellente réactivité : 
Chaque joueur a sa propre "thread" (fil d'exécution). Cela signifie que le jeu peut continuer pour un joueur pendant que le serveur traite les actions des autres joueurs. Par exemple, si un joueur prend du temps pour deviner un nombre, cela n'affecte pas les autres joueurs qui eux ont déjà fait plusieurs guess entre-temps.

- Jeux intéractif = Expérience utilisateur fluide :
En séparant les tâches dans différents threads, nous assurons que les actions comme la lecture des entrées réseau ou les temporisations n'interfèrent pas entre elles. Chaque joueur obtient une réponse rapide.


Gestion des erreurs ==>
Connexion au serveur : Un message d'erreur est affiché si le client ne parvient pas à se connecter au serveur.

Sérialisation/Désérialisation : Les erreurs de sérialisation et de désérialisation des messages sont capturées et traitées avec des messages d'erreur appropriés.

Validation des entrées : Les entrées des utilisateurs sont validées pour s'assurer qu'elles sont conformes aux attentes (par exemple, des nombres pour les devinettes).
