# [EN]
Ren'Py Translation New Key Checker

I didn't like how the files looked when Ren'Py automatically updated translations, so I created a slightly different method.

Normally, Ren'Py would add the new keys to the end of the file, leaving the old ones in case you need to use them (as in the example on the website, a spelling mistake in the source language doesn't usually require changing the translation).
With this program, you input the folder with the completed translation and a newly generated folder. (Rename the translated folder so the engine generates a new one.) A folder called `_replaced` will be generated with the results, plus a .txt file showing the differences, indicating new keys and keys that are no longer present. Each .rpy file will contain your completed translation, with the new keys in their correct positions relative to the rest of the file.
You can process a single file or an entire folder; the processed folder will retain its original structure. In this mode, the diff file will also contain a list of unchanged files, files without a new version, and files without an old version.

Explanation of options:
1. Mode:
Choose whether you want to process a file or a folder. If you choose file mode, the old file will be entered first, followed by the new one.

2. Distribution of diff info:
Choose whether you want each .rpy file to have its own separate diff file, or if you want everything in a single file in the root folder.

3. What to do with unchanged files:
When a file is found to be unchanged, choose whether to copy the translated file to the new _replaced folder, or simply continue with the next one. If a folder had no files with changes and you choose not to copy them, it will not be created, to make it easier to see what changed.

Note: The program will look for changes in the line that declares the key, `translate *language* *key*:`. All subsequent lines will be counted as belonging to that key, broken by the next line comment that usually precedes each declaration, `# game/*path*:*line*`.


# [ES]{Original}
Comprobador de claves nuevas de traduccion de Ren'py.

No me gustaba como quedan los archivos cuando ren'py actualiza las traduciones automaticamente, asi que hice un modo un poco distinto.

Normalmente, ren'py añadiria las nuevas claves al final del archivo y las antiguas se quedarian por si debes usarla (Como el ejemplo en la pagina, un error ortografico en el idioma de origen no suele requerir cambiar la traduccion).

Con este programa, ingresas la carpeta con la traduccion terminada y una carpeta nueva generada. (Cambia el nombre de la carpeta traducida para que el engine genere una nueva) Se generara una carpeta llamada `_replaced` con el resultado, mas un archivo .txt con las diferencias señalando claves nuevas y claves que ya no estan. En cada archivo .rpy tendras tu traduccion terminada, con las nuevas claves en el lugar que les corresponde en relacion al resto del archivo.
Puedes procesar un solo archivo o una carpeta completa, la carpeta procesada mantendra la estructura original. En este modo, el archivo de diferencias contendra tambien una lista de archivos sin cambios, archivos sin version nueva y archivos sin version antigua.

Explicacion de opciones:
1. Modo:
Elije si quieres procesar un archivo o una carpeta. Si elijes modo archivo, lo siguiente que ingresaras el archivo viejo y luego el nuevo.
2. Distribucion de informacion diferencias:
Elije si quieres que cada .rpy tenga su propio archivo de diferencias aparte, o si quieres que todo este en un unico archivo en la carpeta raiz.
3. El que hacer con los archivos sin cambios:
Cuando se encuentre que un archivo no ha tenido cambios, elige si copiar el archivo traducido a la nueva carpeta _replaced, o simplemente continuar con el siguiente. Si una carpeta no tenia archivos con cambios y elijes no copiarlos, esta no se creara, para facilitar ver que cambio.

Nota: El programa buscara cambios en la linea que declara la clave, `translate *language* *key*:`. Se contara como perteneciente a esa clave todas las lineas siguentes, se cortara con el proximo comentario de linea que suele preceder cada declaracion, `# game/*ruta*:*line*`.
