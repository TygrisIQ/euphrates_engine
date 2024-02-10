#version 330 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 incolor;
out vec4 outcolor;

void main(){
  outcolor = vec4(incolor, 1.0f);
  gl_Position = vec4(pos, 1.0f);
}
