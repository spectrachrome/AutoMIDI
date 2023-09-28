# Use an official Python runtime as a parent image
FROM python:3.8-slim

# Set the working directory in docker
WORKDIR /app

# Copy the content of the local src directory to the working directory
COPY midi_recorder.py .

# Install any needed packages specified in requirements.txt
RUN pip install mido python-rtmidi

# Run python script when the container launches
CMD ["python", "midi_recorder.py"]
