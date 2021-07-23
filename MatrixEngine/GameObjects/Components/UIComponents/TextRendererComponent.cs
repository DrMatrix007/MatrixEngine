using SFML.Graphics;

namespace MatrixEngine.GameObjects.Components.UIComponents {
    public class TextRendererComponent : UIRendererComponent {

        public Font font;

        public string text;

        public Text drawable;


        public TextRendererComponent(string text, Font font, Color color, uint char_size = 24, int layer = 50) {
            this.text = text;
            this.font = font;

            drawable = new Text(text, font);

            drawable.CharacterSize = char_size;

            drawable.FillColor = color;



        }


        public override void Render(RenderTarget target) {
            if (drawable.DisplayedString != text) {
                drawable.DisplayedString = text;
            }
            if (drawable.Font != font) {
                drawable.Font = font;
            }

            //+ new Vector2f(0.5f, 0.375f);
            drawable.Position = gameObject.position;


            target.Draw(drawable);

        }

        public override void Start() {
        }
    }
}
