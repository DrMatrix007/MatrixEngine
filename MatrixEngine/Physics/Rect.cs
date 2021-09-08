using System.Collections;
using System.Collections.Generic;
using MatrixEngine.Framework.MathM;
using MatrixEngine.Utilities;
using SFML.System;
using NotImplementedException = System.NotImplementedException;

namespace MatrixEngine.Physics {
    public sealed class Rect {

        public float X, Y, width, height;


        public Vector2f max
        {
            get => new Vector2f(X+width,Y+height); 
        }

        public Vector2f min
        {
            get => new Vector2f(X,Y);
        }
        public float cX { get => X + width * 0.5f; }
        public float cY { get => Y + height * 0.5f; }

        public void SetPos(Vector2f pos) {
            (X,Y) = (pos.X,pos.Y);
        }
        public void SetSize(Vector2f size) {
            (width,height) = (size.X,size.Y);
        }
        public void SetAll(Vector2f pos, Vector2f size) {
            SetSize(size);
            SetPos(pos);
        }
        public Rect(float x = 0, float y = 0, float width = 10, float height = 10) {
            this.X = x;
            this.Y = y;
            this.width = width;
            this.height = height;
        }
        public Rect(Vector2f pos, Vector2f size) {
            SetAll(pos,size);
        }

        public new string ToString() {
            return $"Rect(x:{X.ToString("0.0")}, y:{Y.ToString("0.0")}, width:{width.ToString("0.0")}, height:{height.ToString("0.0")})";
        }
        public Vector2f position
        {
            get { return new Vector2f(X, Y); }
            set { X = value.X; Y = value.Y; }
        }

        public Vector2f center
        {
            get => new Vector2f(cX,cY);
        }
        public Vector2f size
        {
            get => new Vector2f(width,height);
        }

        public bool IsInside(Vector2f pos) {
            
            return (pos.X >= this.X && pos.Y >= this.Y && pos.X<=this.max.X && pos.Y <= this.max.Y);
        
        }
        public Rect Copy() {
            return new Rect(position, size);
        }

        public IEnumerable<Line> ToLines() {
            yield return Line.FromPoints(position,position+size.OnlyWithX());
            yield return Line.FromPoints(position,position+size.OnlyWithY());
            yield return Line.FromPoints(position+size.OnlyWithY(),max);
            yield return Line.FromPoints(position+size.OnlyWithX(),max);
        }
    }
}
