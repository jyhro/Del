# Roadmap (Proximas versiones)

## Tareas implementadas

- [x] Mover archivos/carpetas al trash con sufijo timestamp
- [x] Historial de eliminaciones en archivo (lectura / anexado / reemplazo, formato pipe-delimited)
- [x] Mostrar historial (`--history`) y limpiar historial (`--clear-history`) con confirmación
- [x] Restaurar último o por índice (`-r/--restore [N]`) y manejo de conflictos al restaurar (`_restaurado`)
- [x] Podar entradas obsoletas del historial automáticamente al listar
- [x] Cálculo de tamaño de archivos/carpetas y formato legible (`format_size`)
- [x] Borrado permanente seguro con encriptado en memoria y pasadas aleatorias (PermanentDeleter)
- [x] Confirmaciones interactivas (s/n) para acciones peligrosas
- [x] Sugerencia de bandera desconocida (did-you-mean)
- [x] Mensajes y ayuda en Español con salida coloreada
- [x] Soporte básico por sistema operativo para ubicación del Trash e historial (Windows / Unix)
- [x] Tests unitarios inline para módulos principales

## Mejoras y funcionalidades

- [ ] Mejoras de UX: mensajes mas claros y consistentes, y un resumen final con conteo de archivos movidos, restaurados o eliminados
- [ ] Modo seco ("--dry-run") para simular acciones sin tocar archivos
- [ ] Lista y busqueda avanzada del historial (filtro por fecha, nombre, extension, tamano)
- [ ] Restauracion interactiva con selector por indice
- [ ] Soporte de patrones (glob) y exclusion ("--exclude")
- [ ] Confirmaciones configurables ("--yes", "--no") y recordatorio de la ultima eleccion
- [ ] Integracion con el Trash del sistema en macOS y Linux en lugar de una carpeta propia
- [ ] Limite automatico de espacio/tamano del trash con politica LRU
- [ ] Exportar/importar historial (CSV/JSON)
- [ ] Estadisticas ("del stats"): cantidad de archivos, espacio ahorrado, tendencias por mes
- [ ] Localizacion multi-idioma (manteniendo Espanol como default)
- [ ] Logs detallados con niveles ("--verbose", "--quiet")
- [ ] Soporte de restauracion a carpeta diferente ("--restore-to")
- [ ] Comando "undo" para revertir la ultima eliminacion
- [ ] Mejoras de seguridad en borrado permanente (mas pasadas, patrones, verificacion opcional)
- [ ] Hook de confirmacion con "--force" para entornos CI
- [ ] Tests adicionales de edge cases (paths largos, permisos, links simbolicos)
