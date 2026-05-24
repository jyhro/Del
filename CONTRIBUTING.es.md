# Guia de contribucion (ES)

Gracias por querer contribuir a del.

## Alcance

- del es un binario CLI en Rust (edition 2024).
- No hay lib.rs ni tests de integracion.
- La UI esta en Espanol; los mensajes y prompts deben mantenerse en Espanol.

## Requisitos

- Rust estable instalado.

## Comandos utiles

```bash
cargo build
cargo build --release
cargo test
cargo test <nombre>
cargo run -- --help
```

## Estructura del proyecto

```txt
src/
  main.rs      Entrypoint y wiring
  domain.rs    Tipos y logica de dominio
  output.rs    Toda salida por consola
  cli.rs       Parseo de args y ayuda
  history.rs   IO del historial
  trash.rs     Mover a la papelera
  permanent.rs Borrado permanente
```

## Lineamientos de codigo

- La logica de negocio no debe imprimir ni leer stdin.
- Toda salida va en output.rs.
- Mantener mensajes consistentes y claros.
- Agregar tests inline dentro de cada modulo cuando aplique.

## Cambios y PRs

- Abrir un PR por cambio logico.
- Incluir una descripcion corta y clara.
- Agregar o actualizar tests si corresponde.
- Evitar formateos masivos innecesarios.

## Reporte de bugs

- Incluir sistema operativo.
- Pasos para reproducir.
- Resultado esperado y actual.
