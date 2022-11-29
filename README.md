# gfx-03-b-RGB_Cube
Application (written in bevy/rust-wasm) displaying RGB cube  
You can test it on: https://programistycznyswir.github.io/gfx-03-b-RGB_Cube/  

## Treść zadania:
```md
Ćwiczenie 3 - Przestrzenie barw

b. Rysowanie kostki RGB

Kostka RGB powinna zostać narysowana w trójwymiarze,
Użytkownik powinien mieć możliwość obracania kostką,
Pokrycie kostki kolorami powinno odbywać się przy użyciu odpowiednich wzorów.
```

Uwaga!  
Aplikację, z racji, że kompiluje się do wasm, można przetestować pod adresem https://programistycznyswir.github.io/gfx-03-b-RGB_Cube/  
Główna funkcjonalność jest zawarta w .wgsl'u, skrypt jest bardzo prymitywny,
z tego powodu to kamera obraca się wokół kostki, a nie sama kostka,
żeby to naprawić musisz poprawić musisz w .wgsl offset'ować pozycję
fragmentu względem pozycji kostki, sam ogarnąłem to dopiero jak już oddałem zadanie więc nie chciało mi się poprawiać.
