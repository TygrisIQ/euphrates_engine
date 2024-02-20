#version 330 core

layout (location = 0) in vec3 apos;
layout (location = 1) in vec3 acolor;
layout (location = 2) in vec2 aTexCoord;


out vec2 texcoord;
out vec4 ocolor;

void main(){
  gl_Position = vec4(apos, 1.0f);
  ocolor = vec4(acolor, 1.0f);
  texcoord = aTexCoord;
}
