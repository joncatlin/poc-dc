FROM alpine:latest

EXPOSE 8080

# Create user
RUN addgroup -g 1000 docker-user
RUN adduser -D -s /bin/sh -u 1000 -G docker-user docker-user

# RUN chown myapp:myapp channel_sender
# RUN chown myapp:myapp *.json

USER docker-user

# WORKDIR /home/channel_sender/bin/
WORKDIR /home/docker-user
RUN pwd

# Copy the built executable
COPY --from=cargo-build /usr/src/channel_sender/target/x86_64-unknown-linux-musl/release/channel_sender .
COPY *.json ./

# Create a volume for the templates
RUN mkdir ./templates
VOLUME ["/home/docker-user/templates"]

# Create a volume for the generated pdf files
RUN mkdir ./pdf-output
VOLUME ["/home/docker-user/pdf-output"]

# Create a dir for temp files generated in the pdf process
#RUN mkdir ./pdf-temp-files

CMD ["sleep", "444444444444"]