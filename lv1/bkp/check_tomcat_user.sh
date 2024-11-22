#!/bin/bash

# URL do servidor Tomcat
URL="http://138.197.91.116:8888/manager/html"

# Credenciais do usuário
USERNAME="Kzjhdliw"
PASSWORD="lU56m4fdk045oNf87zMmrFt2D"

# Fazendo a solicitação HTTP
response=$(curl -u $USERNAME:$PASSWORD -s -o /dev/null -w "%{http_code}" $URL)

# Verificando o código de resposta HTTP
if [ $response -eq 200 ]; then
    echo "Usuário $USERNAME existe no servidor Tomcat."
else
    echo "Usuário $USERNAME não existe ou não tem acesso ao servidor Tomcat."
fi