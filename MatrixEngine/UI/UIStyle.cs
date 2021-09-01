using System;
using MatrixEngine.Content;
using SFML.Graphics;

namespace MatrixEngine.UI {
    public class UIStyle {
        public UIStyle(int layer = 0, Color color = default, Color backgroundColor = default) {
            this.layer = layer ;
            this.color = color;
            BackgroundColor = backgroundColor;
        }



        public int layer;
        public Color color;
        public Color BackgroundColor;

        public UIStyle Copy() {
            return MemberwiseClone() as UIStyle;
        }

    }

    public class UITextStyle : UIStyle {
        public UITextStyle(int layer , Color color, Color backgroundColor , Font font ,uint charSize = 10, bool isResize = false) : base(layer, color, backgroundColor) {
            this.font = font;
            char_size = charSize;
            
            is_resize = isResize;
        }



        public Font font;
        public uint char_size;
        public bool is_resize;
        
        
        
    }
}