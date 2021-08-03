using MatrixEngine.GameObjects.Components.PhysicsComponents;
using MatrixEngine.GameObjects.Components.TilemapComponents;
using MatrixEngine.System;
using SFML.System;
using System.Collections.Generic;

namespace MatrixEngine.Physics {
    public class PhysicsEngine {

        public const float Threshold = 0.010f;



        private List<RigidBodyComponent> dynamicRigidBodiesToCalc;
        private List<ColliderComponent> collidersToCalc;

        private List<Rect> rectsToCalc;


        public System.App app
        {
            get;
            private set;
        }

        public PhysicsEngine(System.App app) {
            this.app = app;
            dynamicRigidBodiesToCalc = new List<RigidBodyComponent>();
            collidersToCalc = new List<ColliderComponent>();
            rectsToCalc = new List<Rect>();
        }

        public void AddRigidbodyToFrame(RigidBodyComponent rigidBodyComponent) {

            dynamicRigidBodiesToCalc.Add(rigidBodyComponent);


        }
        public void AddColliderToFrame(ColliderComponent rect) {
            collidersToCalc.Add(rect);
        }

        public void Update() {


            foreach (var nonstatic in dynamicRigidBodiesToCalc) {
                if (!nonstatic.isStatic) {



                }
            }





            var static_list = collidersToCalc.ToArray();
            var non_static_list = dynamicRigidBodiesToCalc.ToArray();


            foreach (var @static in static_list) {

                if (@static.colliderType == ColliderComponent.ColliderType.None) {
                    continue;
                }

                foreach (var nonstatic in non_static_list) {

                    if (nonstatic.colliderComponent.colliderType == ColliderComponent.ColliderType.None) {
                        continue;
                    }

                    if (nonstatic.colliderComponent.colliderType == ColliderComponent.ColliderType.Rect) {

                        if (@static.colliderType == ColliderComponent.ColliderType.Rect) {

                            AddRectToCollision(@static.rect);

                        }
                        if (@static.colliderType == ColliderComponent.ColliderType.Tilemap) {
                            AddTilemapToCollision(nonstatic, @static);
                        }


                    }

                }
            }

            var rs = rectsToCalc.ToArray();

            foreach (var nonstatic in dynamicRigidBodiesToCalc) {
                var add_to_vel = (nonstatic.gravity * app.deltaTime);
                var fric = (nonstatic.velocity.Length() < 1 ? nonstatic.velocity : nonstatic.velocity.Normalize()).Multiply(nonstatic.velocityDrag) * (-1) / 1.0f * app.deltaTime * 100;
                add_to_vel += fric;


                add_to_vel += (nonstatic.gravity * app.deltaTime);

                //add_to_vel;
                //*=app.deltaTime;

                nonstatic.velocity += add_to_vel ;

                //nonstatic.position += (nonstatic.velocity * app.deltaTime);



                nonstatic.position += nonstatic.velocity.OnlyWithX() * app.deltaTime;
                var nonstatic_rect = nonstatic.transform.fullRect;

                foreach (var rect in rectsToCalc) {
                    if (nonstatic_rect.isColliding(rect)) {

                        if(nonstatic_rect.cX<rect.cX) {
                            nonstatic.position = new Vector2f(rect.X - nonstatic_rect.width, nonstatic.position.Y);
                            if (nonstatic.velocity.X> 0) {
                                nonstatic.velocity = nonstatic.velocity.OnlyWithY();
                            }
                        } else {
                            nonstatic.position = new Vector2f(rect.max.X, nonstatic.position.Y);
                            if (nonstatic.velocity.X < 0) {
                                nonstatic.velocity = nonstatic.velocity.OnlyWithY();
                            }
                        }

                    }

                }


                nonstatic.position += nonstatic.velocity.OnlyWithY() * app.deltaTime;
                nonstatic_rect = nonstatic.transform.fullRect;

                foreach (var rect in rectsToCalc) {
                    if (nonstatic_rect.isColliding(rect)) {


                        if (nonstatic_rect.cY < rect.cY) {
                            nonstatic.position = new Vector2f(nonstatic.position.X, rect.Y - nonstatic_rect.height);

                            if (nonstatic.velocity.Y < 0) {
                                nonstatic.velocity = nonstatic.velocity.OnlyWithX();
                            }
                        } else {
                            nonstatic.position = new Vector2f(nonstatic.position.X,rect.max.Y);
                            if (nonstatic.velocity.Y > 0) {
                                nonstatic.velocity = nonstatic.velocity.OnlyWithX();
                            }
                        }

                    }

                }


            }









            dynamicRigidBodiesToCalc.Clear();
            collidersToCalc.Clear();
            rectsToCalc.Clear();
        }

        private void AddTilemapToCollision(RigidBodyComponent nonstatic, ColliderComponent @static) {
            var tilemap = @static.GetComponent<TilemapComponent>();
            if (tilemap == null) {
                return;
            }

            var nonstatic_rect = nonstatic.colliderComponent.rect;
            var tile_scale = tilemap.transform.scale;

            var list_rects = new List<Rect>();

            var pos = new Vector2f(0, 0);

            for (float x = -tile_scale.X * 2; x < nonstatic_rect.width + tile_scale.X * 2; x += tile_scale.X) {
                for (float y = -tile_scale.Y * 2; y < nonstatic_rect.height + tile_scale.Y * 2; y += tile_scale.Y) {
                    pos = new Vector2f(x, y) + tilemap.position;
                    if (tilemap.GetTileFromWorldPos(pos + nonstatic.position) != null) {
                        var r = new Rect((pos + (Vector2f)(Vector2i)nonstatic.position.Round(10)), tile_scale);
                        //if (x == (nonstatic.velocity.X>0?0: tile_scale.X) || y == (nonstatic.velocity.Y > 0 ? tile_scale.Y : 0)) {
                        list_rects.Insert(0, r);
                        //} else {
                        list_rects.Add(r);

                        //}


                    }
                }
            }
            foreach (var item in list_rects) {
               
                AddRectToCollision(item);

            }


        }





        void AddRectToCollision(Rect @static) {

            rectsToCalc.Add(@static);

            


        }
    }

}

