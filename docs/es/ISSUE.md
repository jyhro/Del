# ISSUES

## Issues realizadas

### Mover archivos/carpetas al trash con sufijo timestamp

**Descripcion**
Mueve el item a la carpeta de trash con un nombre unico basado en fecha y hora para evitar colisiones.

### Historial de eliminaciones en archivo (lectura / anexado / reemplazo, formato pipe-delimited)

**Descripcion**
Persiste y recupera el historial en un archivo de texto delimitado por tuberias, soportando lectura, anexado y reemplazo.

### Mostrar historial (--history) y limpiar historial (--clear-history) con confirmacion

**Descripcion**
Lista el historial y permite limpiarlo con confirmacion interactiva para evitar borrados accidentales.

### Restaurar ultimo o por indice (-r/--restore [N]) y manejo de conflictos al restaurar (_restaurado)

**Descripcion**
Restaura el item mas reciente o uno por indice; si el destino ya existe, renombra con sufijo _restaurado.

### Podar entradas obsoletas del historial automaticamente al listar

**Descripcion**
Elimina registros cuyo archivo en trash ya no existe para mantener el historial consistente.

### Calculo de tamano de archivos/carpetas y formato legible (format_size)

**Descripcion**
Calcula tamanos reales de archivos y carpetas y los muestra en unidades legibles.

### Borrado permanente seguro con encriptado en memoria y pasadas aleatorias (PermanentDeleter)

**Descripcion**
Sobreescribe datos con cifrado en memoria y pasadas aleatorias antes de eliminar definitivamente.

### Confirmaciones interactivas (s/n) para acciones peligrosas

**Descripcion**
Solicita confirmacion por stdin antes de operaciones destructivas para prevenir errores.

### Sugerencia de bandera desconocida (did-you-mean)

**Descripcion**
Propone el flag correcto cuando el prefijo coincide con una opcion conocida.

### Mensajes y ayuda en Espanol con salida coloreada

**Descripcion**
Centraliza textos y ayuda de la CLI en Espanol y usa colores para estados.

### Soporte basico por sistema operativo para ubicacion del Trash e historial (Windows / Unix)

**Descripcion**
Define rutas base diferentes por plataforma para trash e historial.

### Tests unitarios inline para modulos principales

**Descripcion**
Incluye pruebas locales en los modulos principales para validar comportamiento base.

## Issues pendientes

### Mejoras de UX: mensajes mas claros y consistentes, y un resumen final con conteo de archivos movidos, restaurados o eliminados

**Descripcion**
Unifica textos y agrega un resumen al final de cada ejecucion con conteos por accion.

### Modo seco ("--dry-run") para simular acciones sin tocar archivos

**Descripcion**
Muestra lo que se haria sin modificar el sistema de archivos ni el historial.

### Lista y busqueda avanzada del historial (filtro por fecha, nombre, extension, tamano)

**Descripcion**
Agrega filtros y ordenamiento para encontrar entradas rapidamente.

### Restauracion interactiva con selector por indice

**Descripcion**
Permite elegir el item a restaurar con un prompt numerado.

### Soporte de patrones (glob) y exclusion ("--exclude")

**Descripcion**
Acepta patrones de archivos y reglas de exclusion al eliminar.

### Confirmaciones configurables ("--yes", "--no") y recordatorio de la ultima eleccion

**Descripcion**
Permite ejecutar en modo no interactivo o repetir decisiones previas.

### Integracion con el Trash del sistema en macOS y Linux en lugar de una carpeta propia

**Descripcion**
Usa la papelera nativa del sistema para mejor compatibilidad y recuperacion.

### Limite automatico de espacio/tamano del trash con politica LRU

**Descripcion**
Limpia automaticamente cuando se supera el limite configurado usando politica LRU.

### Exportar/importar historial (CSV/JSON)

**Descripcion**
Permite respaldar y restaurar el historial en formatos estandar.

### Estadisticas ("del stats"): cantidad de archivos, espacio ahorrado, tendencias por mes

**Descripcion**
Agrega un comando para metricas de uso y tendencias.

### Localizacion multi-idioma (manteniendo Espanol como default)

**Descripcion**
Habilita traducciones con Espanol como idioma por defecto.

### Logs detallados con niveles ("--verbose", "--quiet")

**Descripcion**
Expone niveles de detalle para diagnostico o silencio.

### Soporte de restauracion a carpeta diferente ("--restore-to")

**Descripcion**
Permite restaurar en una ruta distinta a la original.

### Comando "undo" para revertir la ultima eliminacion

**Descripcion**
Atajo para restaurar el item mas reciente con un solo comando.

### Mejoras de seguridad en borrado permanente (mas pasadas, patrones, verificacion opcional)

**Descripcion**
Endurece el borrado con mas pasadas, patrones y verificacion opcional.

### Hook de confirmacion con "--force" para entornos CI

**Descripcion**
Omite prompts interactivos en pipelines y scripts.

### Tests adicionales de edge cases (paths largos, permisos, links simbolicos)

**Descripcion**
Cubre casos limites que fallan en algunos sistemas.
