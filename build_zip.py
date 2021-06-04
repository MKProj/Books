import os
import zipfile
    
def zipdir(path, ziph):
    # ziph is zipfile handle
    for root, dirs, files in os.walk(path):
        for file in files:
            ziph.write(os.path.join(root, file), 
                       os.path.relpath(os.path.join(root, file), 
                                       os.path.join(path, '..')))

book = input("Book Name: ")
all_ = f'{book}_all.zip'
web = f'{book}_web.zip'
pdf = f'{book}_pdf.zip'



zip_all = zipfile.ZipFile(f'Releases/{all_}', 'w', zipfile.ZIP_DEFLATED)
zip_web = zipfile.ZipFile(f'Releases/{web}', 'w', zipfile.ZIP_DEFLATED)
zip_pdf = zipfile.ZipFile(f'Releases/{pdf}', 'w', zipfile.ZIP_DEFLATED)
zipdir(f'{book}/', zip_all)
zipdir(f'{book}/book/', zip_web)
zipdir(f'{book}/Tex/', zip_pdf)

zip_all.close()
zip_web.close()
zip_pdf.close()