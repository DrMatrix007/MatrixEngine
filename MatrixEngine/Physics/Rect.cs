using SFML.System;

namespace MatrixEngine.Physics {
    public sealed class Rect {

        public float x, y, width, height;


        public Vector2f max
        {
            get => new Vector2f(x+width,y+height); 
        }

        public Vector2f min
        {
            get => new Vector2f(x,y);
        }
        public float cX { get => x + width * 0.5f; }
        public float cY { get => y + height * 0.5f; }

        public void SetPos(Vector2f pos) {
            (x,y) = (pos.X,pos.Y);
        }
        public void SetSize(Vector2f size) {
            (width,height) = (size.X,size.Y);
        }
        public void SetAll(Vector2f pos, Vector2f size) {
            SetSize(size);
            SetPos(pos);
        }
        public Rect(float x = 0, float y = 0, float width = 10, float height = 10) {
            this.x = x;
            this.y = y;
            this.width = width;
            this.height = height;
        }
        public Rect(Vector2f pos, Vector2f size) {
            SetAll(pos,size);
        }

        public new string ToString() {
            return $"Rect(x:{x.ToString("0.0")}, y:{y.ToString("0.0")}, width:{width.ToString("0.0")}, height:{height.ToString("0.0")})";
        }
        public Vector2f position
        {
            get { return new Vector2f(x, y); }
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
            
            return (pos.X >= this.x && pos.Y >= this.y && pos.X<=this.max.X && pos.Y <= this.max.Y);
        
        }

    }
}
