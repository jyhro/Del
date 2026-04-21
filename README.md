# del

**del** es una utilidad de línea de comandos escrita en Rust para eliminar archivos y carpetas de forma segura (enviándolos a una papelera) o permanente, inspirada en la funcionalidad de la papelera de sistemas operativos modernos.

## Características

- Elimina archivos y carpetas moviéndolos a una papelera (`~/.local/share/Trash`)
- Permite restaurar el último archivo/carpeta eliminado
- Elimina de forma permanente con confirmación
- Listado del contenido de la papelera
- Historial de eliminaciones
- Salida colorizada y mensajes en español
- Modular y fácil de mantener (principios SOLID)

## Instalación

1. Clona el repositorio:

   ```sh
   git clone <repo-url>
   cd del
   ```

2. Compila el proyecto:

   ```sh
   cargo build --release
   ```

3. (Opcional) Copia el binario a una ruta en tu `$PATH`:

   ```sh
   cp target/release/del ~/.local/bin/
   ```

## Uso

```sh
del [opciones] <archivo/carpeta> [...]
del -p, --permanent <archivo/carpeta>  # Elimina permanentemente
 del -r, --restore                     # Restaura el último archivo/carpeta
 del --list                            # Lista el contenido de la papelera
 del --help                            # Muestra la ayuda
```

### Ejemplos

- Eliminar seguro:

  ```sh
  del archivo.txt
  ```

- Eliminar permanente:

  ```sh
  del -p archivo.txt
  ```

- Listar papelera:

  ```sh
  del --list
  ```

- Restaurar último:

  ```sh
  del -r
  ```

## Estructura del proyecto

- `src/main.rs`: Orquestador principal y CLI
- `src/trash/manager.rs`: Lógica de papelera (mover, restaurar, listar)
- `src/permanent.rs`: Lógica de eliminación permanente

## Licencia

MIT

## Autor

del Rust port
