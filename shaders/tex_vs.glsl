#version 440 core

layout (location = 0) in vec3 apos;
layout (location = 1) in vec2 aTexCoord;


out vec2 texcoord;


void main(){
  gl_Position = vec4(apos.x, apos.y, apos.z, 1.0f);
  texcoord = aTexCoord;
}
