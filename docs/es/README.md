# del

Un CLI pequeño y multiplataforma para eliminar archivos de forma más segura en Unix/macOS y Windows.

Objetivos principales: mover archivos a una papelera local, llevar un historial para restauraciones sencillas y ofrecer una opción de borrado permanente y seguro.

## Puntos destacados

- Papelera local en vez de borrado inmediato e irreversible
- Restaurar por índice de historial o restaurar la eliminación más reciente
- Registrar timestamps y tamaños en un archivo de historial compacto
- Opción de sobrescritura segura (`-p`) para borrados irreversibles
- Modo simulación (`--dry-run`) para previsualizar acciones sin tocar archivos
- Salida de terminal amigable mediante `rich_rust`

## Ejemplos rápidos

Eliminar un solo archivo a la papelera:

```bash
del archivo.txt
```

Eliminar varios archivos o directorios:

```bash
del archivo1.txt archivo2.txt mi_carpeta/
```

Simular una eliminación sin mover ni borrar archivos:

```bash
del --dry-run archivo.txt
```

Restaurar la última entrada eliminada:

```bash
del -r
```

Restaurar una entrada específica del historial (índice 1-based):

```bash
del -r 3
```

Listar el historial y metadatos:

```bash
del --history
```

Borrar de forma permanente y segura un archivo (requiere confirmación):

```bash
del -p sensible.log
```

Mostrar ayuda:

```bash
del --help
```

## Instalación

Desde el código fuente:

```bash
git clone https://github.com/jyhro/Del.git
cd Del
./install.sh          # Unix/macOS
# o
./install.ps1         # Windows (PowerShell)
```

Desde crates.io:

```bash
cargo install del
```

## Cómo funciona (resumen)

- Directorio de la papelera (dependiente de la plataforma): los archivos se mueven aquí en lugar de eliminarse.
- Los archivos se renombran con un sufijo de timestamp para evitar colisiones.
- Un archivo de historial compacto y delimitado por tuberías registra: `original_path|file_name|trash_path|timestamp|size`.

Valores por plataforma:

- macOS: Papelera `~/.Trash` — Historial `~/.del_history`
- Unix: Papelera `~/.local/share/Trash` — Historial `~/.local/share/del_history`
- Windows: Papelera `%USERPROFILE%\\AppData\\Local\\Temp\\Trash` — Historial `%USERPROFILE%\\AppData\\Local\\del_history`

### Borrado permanente (`-p`)

El borrado permanente realiza los siguientes pasos (comportamiento seguro por defecto):

1. Pase XOR en memoria usando randomness de `OsRng` (ofuscación en RAM)
2. Dos pasadas de sobrescritura con datos criptográficamente aleatorios sobre el contenido del fichero
3. Eliminación del fichero del disco

Los archivos de longitud cero se eliminan directamente sin sobrescritura. Los directorios se procesan recursivamente y cada archivo se borra individualmente.

## Estructura del proyecto

```txt
src/
├── main.rs        # Punto de entrada y wiring (Console + Summary lifecycle)
├── domain.rs      # Tipos de dominio, errores, contadores Summary
├── output.rs      # Salida en terminal y prompts (UI en español)
├── cli.rs         # Parseo de argumentos → `Command` enum
├── trash.rs       # Mover/restaurar + implementación del repositorio de historial
└── permanent.rs   # Lógica de sobrescritura segura y eliminación
```

Nota de diseño: la lógica de negocio nunca lee stdin ni imprime directamente — `output.rs` gestiona toda la E/S y el formateo.

## Desarrollo

Compilar y ejecutar localmente:

```bash
cargo build           # compilación en modo debug
cargo build --release # compilación en modo release
cargo run -- <args>   # ejecutar la CLI
```

Ejecutar tests:

```bash
cargo test            # ejecutar pruebas unitarias (inline)
cargo test <nombre>   # ejecutar una sola prueba por nombre
```

## Contribuir

Las contribuciones son bienvenidas. Consulta las guías de contribución para flujo de trabajo y estilo:

- [CONTRIBUTING](CONTRIBUTING.md)

Cuando agregues funciones que cambien textos visibles por el usuario, ten en cuenta que este proyecto usa actualmente cadenas de UI en español en `output.rs`.

## Hoja de ruta e incidencias

Consulta [ROADMAP.md](../ROADMAP.md) para mejoras planificadas. Reporta errores o solicitudes de característica mediante issues en GitHub.

## Licencia

Este proyecto está bajo la licencia MIT — ver [LICENSE](../LICENSE).

## Agradecimientos

- `rich_rust` para el formateo en terminal
- Inspiración: flujos de trabajo tradicionales de papelera y restauración
