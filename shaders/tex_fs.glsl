#version 440 core

out vec4 Fragcolor;
in vec2 texcoord;

uniform sampler2D ourtexture;
void main(){
  Fragcolor = texture(ourtexture, texcoord);
}
