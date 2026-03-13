FROM r-base:4.3.3
RUN apt-get update && apt-get install -y libssl-dev libcurl4-openssl-dev libxml2-dev && rm -rf /var/lib/apt/lists/*
RUN R -e "install.packages(c('shiny','shinydashboard','rvest','dplyr','stringr','lubridate','plotly','DT'), repos='https://cloud.r-project.org/')"
WORKDIR /app
COPY app.R .
EXPOSE 3838
CMD ["R", "-e", "shiny::runApp('/app', host='0.0.0.0', port=3838)"]
